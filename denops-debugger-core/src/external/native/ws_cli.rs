use v8_inspector_api_types::commands;
use v8_inspector_api_types::prelude::Method;
use ws::{connect, CloseCode};

// impl Transport {
//     pub fn call_method<C>(
//         &self,
//         method: C,
//         destination: MethodDestination,
//     ) -> Fallible<C::ReturnObject>
//     where
//         C: protocol::Method + serde::Serialize,
//     {
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

pub fn ws_connection(url: String) {
    let a = commands::Enable {};
    let data = a.into_method_call(1);
    connect(url, |out| {
        out.send(serde_json::to_string(data.as_ref()).unwrap().as_str())
            .unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    })
    .unwrap()
}
