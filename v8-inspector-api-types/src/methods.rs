use crate::types::JsUInt;
use serde::{Deserializer, Serialize};
use std::fmt::Debug;

/// Serialized struct for method call
#[derive(Serialize, Debug)]
pub struct MethodCall<T> {
    #[serde(rename = "method")]
    method_name: &'static str,
    pub id: JsUInt,
    params: T,
}

impl<T> MethodCall<T> {
    pub fn get_params(&self) -> &T {
        &self.params
    }
}

pub trait Method {
    const NAME: &'static str;

    type ReturnObject: serde::de::DeserializeOwned;

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
