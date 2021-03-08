use ws::{connect, CloseCode};

pub fn ws_connection(url: String) {
    connect(url, |out| {
        out.send("Hello WebSocket").unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    })
    .unwrap()
}
