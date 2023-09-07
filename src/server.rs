use std::{io::ErrorKind, time::Duration};
use tokio::time::interval;
use warp::{reply::Reply, Filter};

const CONFIG_FILE: &str = "extensao.json";
const DEFAULT_CONFIG_FILE: &str = r#"{"ip":"0.0.0.0","port":4040}"#;

#[derive(serde::Deserialize)]
struct Config {
    ip: String,
    port: u16,
}

pub fn serve(shutdown: Option<tokio::sync::oneshot::Receiver<()>>) {
    let api_create = warp::post()
        .and(warp::path("sala"))
        .and(warp::path::end())
        .map(crate::api::api_create);

    let api_leave = warp::post()
        .and(warp::path("sala"))
        .and(warp::path("sair"))
        .and(warp::path::end())
        .map(crate::api::api_leave);

    let api_join = warp::post()
        .and(warp::path("sala"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .map(crate::api::api_join);

    let api_connect = warp::ws()
        .and(warp::path("sala"))
        .and(warp::path::end())
        .and(warp::cookie::<String>("session"))
        .map(|ws, session| {
            if let Some((roomid, sckid)) = parse_session(session) {
                crate::api::api_connect(ws, roomid, sckid)
            } else {
                warp::reply::with_status(warp::reply::reply(), warp::http::StatusCode::BAD_REQUEST)
                    .into_response()
            }
        });

    let apis = api_create.or(api_leave).or(api_join).or(api_connect);

    #[cfg(not(debug_assertions))] // load assets from executable
    let files = static_dir::static_dir!("static");

    #[cfg(debug_assertions)] // load assets from static directory
    let files = warp::fs::dir("static");

    let index = warp::get().and(
        warp::path("index.html")
            .or(warp::any())
            .unify()
            .and(warp::path::end())
            .and(
                warp::cookie::<String>("session")
                    .or(warp::any().map(String::new))
                    .unify(),
            )
            .map(|session| crate::api::index(parse_session(session)))
            .or(warp::path("master.html")
                .or(warp::path("member.html"))
                .and(warp::path::end())
                .map(|_| warp::redirect::see_other(warp::http::Uri::from_static("index.html")))),
    );

    let routes = index.or(files).or(apis);

    // cd into folder of executable, for reading the correct config file
    #[cfg(not(debug_assertions))]
    std::env::set_current_dir(
        std::env::current_exe()
            .expect("current_exe")
            .parent()
            .expect("parent"),
    )
    .expect("set_current_dir");

    let config = match std::fs::metadata(CONFIG_FILE) {
        Ok(metadata) => metadata.is_file(),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            std::fs::write(CONFIG_FILE, DEFAULT_CONFIG_FILE).is_ok()
        }
        Err(_) => false,
    };

    if !config {
        println!(
            "[!] ERROR: default \"{}\" file was not found and could not be created",
            CONFIG_FILE
        );
        return;
    }

    let config = match std::fs::read_to_string(CONFIG_FILE) {
        Ok(config) => config,
        Err(_) => {
            println!("[!] ERROR: could not read config file");
            return;
        }
    };

    let Config { ip, port } = match serde_json::from_str(&config) {
        Ok(config) => config,
        Err(_) => {
            println!("[!] ERROR: config file is not a valid json config file");
            return;
        }
    };

    let Some(ip) = parse_ip(&ip) else {
        println!("[!] ERROR: ip \"{}\" is not valid", ip);
        return;
    };

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build runtime");

    let _enter = rt.enter();

    // Run a function every second
    tokio::spawn(async move {
        let mut tick = 0;
        let mut tick_task = interval(Duration::from_secs(1));
        tick_task.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            crate::state::periodic_routine(tick);
            tick = tick.wrapping_add(1);
            tick_task.tick().await;
        }
    });

    rt.block_on(
        warp::serve(routes)
            .bind_with_graceful_shutdown((ip, port), async move {
                if ip == [0, 0, 0, 0] {
                    println!("[*] Projeto de Extensao *:{}", port);
                } else {
                    println!(
                        "[*] Projeto de Extensao {}.{}.{}.{}:{}",
                        ip[0], ip[1], ip[2], ip[3], port
                    );
                }
                if let Some(shutdown) = shutdown {
                    shutdown
                        .await
                        .expect("The shutdown oneshot chanel's sender must not be dropped");
                    println!("[*] Stopping service");
                } else {
                    if let Err(_) = tokio::signal::ctrl_c().await {
                        println!("[!] Failed to detect CTRL-C");
                        // the line below never returns
                        let () = std::future::pending().await;
                    }
                    println!("[*] CTRL-C detected");
                }
            })
            .1,
    );
}

fn parse_ip(ip: &str) -> Option<[u8; 4]> {
    if ip.is_empty() {
        return Some([0, 0, 0, 0]);
    }
    let parts: Vec<&str> = ip.splitn(5, '.').collect();
    if parts.len() != 4 {
        return None;
    }
    Some([
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
        parts[3].parse().ok()?,
    ])
}

fn parse_session(mut session: String) -> Option<(String, u32)> {
    let index = session.find(':')?;
    let sckid = session[index + 1..].parse().ok()?;
    session.truncate(index);
    Some((session, sckid))
}
