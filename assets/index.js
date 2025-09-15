async function greetUser() {
    const name = document.getElementById('nameInput').value || 'World';
    try {
        const response = await window.rpc.postMessage(JSON.stringify({ type: 'Greet', parameters: { name: name } }));
        console.debug(response);
        document.getElementById('response').textContent = JSON.stringify(response, null, 2);
    } catch (error) {
        document.getElementById('response').textContent = 'Error: ' + error;
    }
}

let count = 0;
async function incrementCounter() {
    let request = { type: 'IncrementCounter', parameters: { value: count } };
    console.info(`request: ${request}`);
    let response = await window.rpc.postMessage(JSON.stringify(request));
    console.warn(`response: ${response}`);

    count = response.value
    document.getElementById('counter').textContent = count;

    request = { type: 'UpdateCounter', parameters: { value: count } };
    console.info(`request: ${request}`);
    response = await window.rpc.postMessage(JSON.stringify(request));
    console.warn(`response: ${response}`);
}
async function decrementCounter() {
    const request = { type: 'DecrementCounter', parameters: { value: count } };
    console.info(`request: ${request}`);
    const response = await window.rpc.postMessage(JSON.stringify(request));
    console.warn(`response: ${response}`);

    count = response.value
    document.getElementById('counter').textContent = count;

    request = { type: 'UpdateCounter', parameters: { value: count } };
    console.info(`request: ${request}`);
    response = await window.rpc.postMessage(JSON.stringify(request));
    console.warn(`response: ${response}`);
}

async function getCurrentTime() {
    try {
        const response = await window.rpc.postMessage(JSON.stringify({ type: 'GetTime' }));
        document.getElementById('response').textContent = JSON.stringify(response, null, 2);
    } catch (error) {
        document.getElementById('response').textContent = 'Error: ' + error;
    }
}

document.addEventListener('DOMContentLoaded', function () {
    console.log('Rust Webview Demo loaded!');
    document.getElementById('greet-user').addEventListener('click', greetUser);
    document.getElementById('decrement-counter').addEventListener('click', decrementCounter);
    document.getElementById('increment-counter').addEventListener('click', incrementCounter);
    document.getElementById('get-current-time').addEventListener('click', getCurrentTime);
});
