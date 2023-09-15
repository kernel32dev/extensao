
const api_websocket = (window.location.port
    ? `ws://${window.location.hostname}:${window.location.port}/sala`
    : `ws://${window.location.hostname}/sala`
);

/** faz um request post para o url especificado, e chama callback com o
 * resultado */
function post(url, callback) {
    const xhr = new XMLHttpRequest();
    xhr.open("POST", url, true);
    xhr.withCredentials = true;
    xhr.onload = function () {
        if (xhr.status >= 200 && xhr.status < 300) {
            console.log(`POST ${xhr.status} ${url} => ${xhr.responseText}`);
            if (callback) callback(xhr.responseText);
        } else {
            console.error(`POST ${xhr.status} ${url} => ${xhr.statusText}`);
        }
    };
    xhr.onerror = function (error) {
        console.error(error);
    };
    xhr.send();
}

/** cria uma nova conecção websocket, se for desconectado, reconecta
 * automaticamente */
function connection(callback) {
    let ws = new WebSocket(api_websocket);
    ws.onerror = onerror;
    ws.onmessage = onmessage;
    function onerror(error) {
        ws.close();
        setTimeout(function () {
            if (ws === null) return;
            ws = new WebSocket(api_websocket);
            ws.onerror = onerror;
            ws.onmessage = onmessage;
        }, 3000);
    }
    function onmessage(ev) {
        if (typeof ev.data === "object") {
            ev.data.text().then(function (text) {
                let json = JSON.parse(text);
                console.log("<<<", json);
                if (typeof callback === "function") {
                    callback(json);
                }
            }).catch(function (error) {
                console.error(error);
            });
        } else {
            let json = JSON.parse(ev.data);
            console.log("<<<", json);
            callback(json);
        }
    }
    return {
        send: function (data) {
            console.log(">>>", data);
            ws.send(JSON.stringify(data));
        },
        close: function () {
            ws.close();
            ws.onerror = undefined;
            ws.onmessage = undefined;
        }
    }
}

/** esconde todos os elementos com a classe page, exceto os que tem os ids
 * especificados */
function show_page() {
    let pages = document.getElementsByClassName("page");
    for (let i = 0; i < pages.length; i++) {
        pages[i].classList.add("hide");
    }
    for (let i = 0; i < arguments.length; i++) {
        if (typeof arguments[i] === "string") {
            let page = document.getElementById(arguments[i]);
            page.classList.remove("hide");
        }
    }
}

/** retorna o código da sala e sckid atualmente sendo usados, caso uma sessão
 * já tenha sido criada, serve para voltar para onde estavamos depois de um
 * refresh */
function get_session() {
    const cookies = document.cookie.split(';');
    for (let i = 0; i < cookies.length; i++) {
        const cookie = cookies[i].trim();
        const cookieParts = cookie.split('=');
        const cookieName = decodeURIComponent(cookieParts[0]);
        if (cookieName === "session") {
            const cookieValue = decodeURIComponent(cookieParts[1]);
            const sessionParts = cookieValue.split(":");
            const roomid = sessionParts[0];
            const sckid = Number(sessionParts[1]);
            return { roomid, sckid };
        }
    }
    return null;
}

function create_table(value) {
    if (typeof value !== "object") {
        return document.createTextNode(String(value));
    } else if (value instanceof Array) {
        return create_table_array(value);
    } else {
        return create_table_obj(value);
    }
}

function create_table_obj(obj) {
    let table = document.createElement("table");
    let tbody = document.createElement("tbody");
    table.append(tbody);
    let entries = Object.entries(obj);
    for (let i = 0; i < entries.length; i++) {
        let tr = document.createElement("tr");
        let cell_key = document.createElement("th");
        let cell_value = document.createElement("td");
        tbody.append(tr);
        tr.append(cell_key, cell_value);
        cell_key.innerText = entries[i][0];
        let value = entries[i][1];
        cell_value.append(create_table(value));
    }
    return table;
}

function create_table_array(array) {
    if (array.length === 0) {
        let span = document.createElement("span");
        span.innerHTML = "&varnothing;";
        return span;
    }
    let table = document.createElement("table");
    let thead = document.createElement("thead");
    let tbody = document.createElement("tbody");
    let thead_row = document.createElement("tr");
    table.append(thead, tbody);
    thead.append(thead_row);
    let keys = Object.keys(array[0]);
    for (let i = 0; i < keys.length; i++) {
        let th = document.createElement("th");
        th.innerText = keys[i];
        thead_row.append(th);
    }
    for (let i = 0; i < array.length; i++) {
        let tr = document.createElement("tr");
        tbody.append(tr);
        for (let j = 0; j < keys.length; j++) {
            let value = array[i][keys[j]];
            let td = document.createElement("td");
            tr.append(td);
            if (typeof value === "object") {
                if (value instanceof Array) {
                    td.append(create_table_array(value));
                } else {
                    td.append(create_table_obj(value));
                }
            } else {
                td.innerText = value;
            }
        }
    }
    return table;
}

function replace_children(parent, child) {
    parent.innerHTML = "";
    parent.append(child);
}

/** inicia um count down com o número especificado de segundos
 * callback será chamado todo segundo com uma string no formato "0:00",
 * chamando as funções retornadas, é possível parar o count down e adicionar
 * mais tempo*/
function startCountDown(duration, callback) {
    let timer = duration;
    let id = setInterval(tick, 1000);
    tick();
    return {
        stop: function () {
            clearInterval(id);
        },
        extra: function (seconds) {
            timer += seconds;
            tick();
        },
    };
    function tick() {
        if (timer <= 0) {
            return;
        }

        timer--;

        let minutes = parseInt(timer / 60, 10);
        let seconds = parseInt(timer % 60, 10);

        //minutes = minutes < 10 ? "0" + minutes : minutes;
        minutes = String(minutes);
        seconds = seconds < 10 ? "0" + seconds : seconds;

        callback(minutes + ":" + seconds);
    }
}
