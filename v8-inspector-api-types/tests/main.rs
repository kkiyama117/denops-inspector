use v8_inspector_api_types::messages::Message;
// TODO: Add tests

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use v8_inspector_api_types::messages::Message;

    // #[test]
    // fn pass_through_channel() {
    //     // let attached_to_target_json = json!({
    //     //     "method": "Target.attachedToTarget",
    //     //     "params": {
    //     //         "sessionId": "8BEF122ABAB0C43B5729585A537F424A",
    //     //         "targetInfo": {
    //     //             "targetId": "26DEBCB2A45BEFC67A84012AC32C8B2A",
    //     //             "type": "page",
    //     //             "title": "",
    //     //             "url": "about:blank",
    //     //             "attached": true,
    //     //             "browserContextId": "946423F3D201EFA1A5FCF3462E340C15"
    //     //         },
    //     //         "waitingForDebugger": false
    //     //     }
    //     // });
    //     //
    //     // let _event: Message = serde_json::from_value(attached_to_target_json).unwrap();
    // }

    #[test]
    fn parse_event_fully() {
        // let attached_to_target_json = json!({
        //     "method": "Target.attachedToTarget",
        //     "params": {
        //         "sessionId": "8BEF122ABAB0C43B5729585A537F424A",
        //         "targetInfo": {
        //             "targetId": "26DEBCB2A45BEFC67A84012AC32C8B2A",
        //             "type": "page",
        //             "title": "",
        //             "url": "about:blank",
        //             "attached": true,
        //             "browserContextId": "946423F3D201EFA1A5FCF3462E340C15"
        //         },
        //         "waitingForDebugger": false
        //     }
        // });
        //
        // if let Ok(Event::AttachedToTarget(_)) = serde_json::from_value(attached_to_target_json) {
        // } else {
        //     panic!("Failed to parse event properly");
        // }
        //
        // let received_target_msg_event = json!({
        //     "method": "Target.receivedMessageFromTarget",
        //     "params": {
        //         "sessionId": "8BEF122ABAB0C43B5729585A537F424A",
        //         "message": "{\"id\":43473,\"result\":{\"data\":\"kDEgAABII=\"}}",
        //         "targetId": "26DEBCB2A45BEFC67A84012AC32C8B2A"
        //     }
        // });
        // let event: Event = serde_json::from_value(received_target_msg_event).unwrap();
        // match event {
        //     Event::ReceivedMessageFromTarget(ev) => {
        //         println!("{:?}", ev);
        //     }
        //     _ => panic!("bad news"),
        // }
    }

    #[test]
    fn easy_parse_messages() {
        // let example_message_strings = [""];
        //
        // for msg_string in &example_message_strings {
        //     let _message: super::Message = parse_raw_message(msg_string).unwrap();
        // }
    }
}

fn parse_raw_message(raw_message: &str) -> anyhow::Result<Message> {
    Ok(serde_json::from_str::<Message>(raw_message)?)
}
