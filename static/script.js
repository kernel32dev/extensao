
const api_websocket = (window.location.port
    ? `ws://${window.location.hostname}:${window.location.port}/sala`
    : `ws://${window.location.hostname}/sala`
);

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

/** atualiza um elemento tbody com o row, procura um elemento com a chave
 * especificada e atualiza ele, se não existir, cria um novo */
function update_tbody(tbody, row, key, columns) {
    let html = "";
    for (let i = 0; i < columns.length; i++) {
        html += "<td>" + row[columns[i]] + "</td>";
    }
    for (let tr = tbody.firstElementChild; tr; tr = tr.nextElementSibling) {
        if (tr.dataset.key == row[key]) {
            tr.innerHTML = html;
            return;
        }
    }
    let tr = document.createElement("tr");
    tr.dataset.key = row[key];
    tr.innerHTML = html;
    tbody.append(tr);
}

/** procura um elemento com a chave especificada e apaga ele, faz nada se o
 * elemento não for encontrado */
function update_tbody_delete(tbody, key) {
    for (let tr = tbody.firstElementChild; tr; tr = tr.nextElementSibling) {
        if (tr.dataset.key == key) {
            tr.remove();
            break;
        }
    }
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