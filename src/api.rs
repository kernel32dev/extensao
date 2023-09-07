use crate::state;
use futures::{SinkExt, StreamExt, TryStreamExt};
use warp::reply::{Reply, Response};

pub fn set_cookie(room: &str, sckid: u32) -> String {
    format!("session={room}:{sckid}; max-age=3600; path=/;")
}
pub fn unset_cookie() -> &'static str {
    "session=; max-age=0; path=/;"
}

pub fn index(session: Option<(String, u32)>) -> Response {
    #[cfg(not(debug_assertions))]
    let index = || include_str!("../static/index.html");

    #[cfg(debug_assertions)]
    let index = || std::fs::read_to_string("static/index.html").unwrap_or_default();

    if let Some((roomid, sckid)) = session {
        if !crate::state::check_exists(&roomid, sckid) {
            return warp::reply::with_header(warp::reply::html(index()), "set-cookie", unset_cookie())
                .into_response();
        }
    }

    warp::reply::html(index()).into_response()
}

pub fn api_create() -> Response {
    match state::create_room() {
        Ok(room) => {
            let set_cookie = set_cookie(&room, 0);
            warp::reply::with_header(
                warp::reply::html(room),
                "set-cookie",
                set_cookie,
            )
            .into_response()
        }
        Err(()) => warp::reply::with_status(
            warp::reply::html("Failed to create the room, presumably, it must be full"),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

pub fn api_join(room: String) -> Response {
    match state::join_room(&room) {
        Ok(sckid) => {
            let set_cookie = set_cookie(&room, sckid);
            warp::reply::with_header(
                warp::reply::with_header(
                    warp::reply::html(sckid.to_string()),
                    "content-type",
                    "text/plain",
                ),
                "set-cookie",
                set_cookie,
            )
            .into_response()
        }
        Err(()) => warp::reply::with_status(
            warp::reply::html("Room does not exist, or the game is running"),
            warp::http::StatusCode::NOT_FOUND,
        )
        .into_response(),
    }
}

pub fn api_leave() -> Response {
    warp::reply::with_header(warp::reply::reply(), "set-cookie", unset_cookie()).into_response()
}

pub fn api_connect(ws: warp::ws::Ws, room: String, sckid: u32) -> Response {
    #[cfg(debug_assertions)]
    const DEBUG_WEB_SOCKET: bool = true;
    #[cfg(not(debug_assertions))]
    const DEBUG_WEB_SOCKET: bool = false;
    match state::connect_room(&room, sckid) {
        Ok(mut receiver) => ws
            .on_upgrade(move |ws| async move {
                if DEBUG_WEB_SOCKET {
                    println!("[*] Websocket stream spawned");
                }
                let room = room;
                let _ = state::increment_online(&room, sckid);
                let (mut sink, stream) = ws.split();
                let sink_handler = async {
                    while let Some(message) = receiver.recv().await {
                        if sink.send(message).await.is_err() {
                            break;
                        }
                    }
                };
                let another_room = room.clone();
                let mut stream_handler = stream
                    .map(move |result| {
                        println!("[*] Websocket message received! ok: {}", result.is_ok());
                        result.map(|x| {
                            if x.is_close() {
                                return true;
                            }
                            if let Err(error) = state::handle_message(&another_room, sckid, x) {
                                if DEBUG_WEB_SOCKET {
                                    println!("[*] Websocket message handler error: {}", error.err);
                                }
                            }
                            false
                        })
                    })
                    .try_filter(|x| std::future::ready(*x))
                    .map(|x| x.map(|_| ()));
                let stream_handler = async { stream_handler.try_next().await.map(|_| ()) };
                if let Err(error) =
                    tokio::select!(() = sink_handler => Ok(()), b = stream_handler => b)
                {
                    println!("[*] Websocket error: {:?}", error);
                }
                let _ = sink.close().await;
                let _ = state::decrement_online(&room, sckid);
                if DEBUG_WEB_SOCKET {
                    println!("[*] Websocket stream closed");
                }
            })
            .into_response(),
        Err(()) => {
            println!("[!] Room/sckid does not exist");
            warp::reply::with_header(
                warp::reply::with_status(
                    warp::reply::html("Room/sckid does not exist"),
                    warp::http::StatusCode::NOT_FOUND,
                ),
                "set-cookie",
                unset_cookie(),
            )
            .into_response()
        }
    }
}
