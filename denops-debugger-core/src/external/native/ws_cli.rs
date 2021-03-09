use ws::{connect, CloseCode};

pub fn ws_connection(url: String) {
    let data = r#"{"method": "Debugger.enable","params": null, "id": 1}"#;
    connect(url, |out| {
        out.send(data).unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    })
    .unwrap()
}
