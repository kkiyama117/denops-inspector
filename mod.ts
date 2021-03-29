const url = Deno.args[0];
const ws = new WebSocket(url);
import {Protocol} from "./deps.ts"

// Register event listeners for the open, close, and message events.rs
ws.onopen = () => {
    console.log("WebSocket ready!");
    let msg = JSON.stringify({
        id: 0,
        method: 'Debugger.enable',
    });
    ws.send(msg);
    msg = JSON.stringify({
        id: 1,
        method: 'Debugger.getScriptSource',
    });
    ws.send(msg);
};
ws.onmessage = (message) => {
    console.log("Received data:", message.data);
    ws.close();
};
ws.onclose = () => console.log("WebSocket closed!");
ws.onerror = (err) => console.log("WebSocket error:", err);

// When running this the following is logged to the console:
//
// WebSocket ready!
// Received data: Hello World!
// WebSocket closed!