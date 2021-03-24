use crate::types::JsUInt;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Debug)]
pub struct MethodCall<T>
where
    T: Debug,
{
    #[serde(rename = "method")]
    method_name: &'static str,
    pub id: JsUInt,
    params: T,
}

impl<T> MethodCall<T>
where
    T: Debug,
{
    pub fn get_params(&self) -> &T {
        &self.params
    }
}

pub trait Method: Debug {
    const NAME: &'static str;

    type ReturnObject: serde::de::DeserializeOwned + std::fmt::Debug;

    fn into_method_call(self, call_id: JsUInt) -> Box<MethodCall<Self>>
    where
        Self: std::marker::Sized,
    {
        Box::new(MethodCall {
            id: call_id,
            params: self,
            method_name: Self::NAME,
        })
    }
}

/// Debugger.Enable
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enable {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl Method for Enable {
    const NAME: &'static str = "Debugger.enable";
    type ReturnObject = EnableReturnObject;
}
