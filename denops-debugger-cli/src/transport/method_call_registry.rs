use crate::types::CallId;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::watch::Sender;
use v8_inspector_api_types::messages::Response;

#[derive(Debug)]
pub struct WaitingCallRegistry {
    calls: Mutex<HashMap<CallId, Sender<anyhow::Result<Response>>>>,
}
