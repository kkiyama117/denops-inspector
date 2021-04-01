// pub mod events;
// pub mod methods;

use serde::Deserialize;

pub type TargetId = String;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    Page,
    BackgroundPage,
    ServiceWorker,
    Browser,
    Other,
}

impl TargetType {
    pub fn is_page(&self) -> bool {
        match self {
            Self::Page => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    pub target_id: TargetId,
    #[serde(rename = "type")]
    pub target_type: TargetType,
    pub title: String,
    pub url: String,
    pub attached: bool,
    pub opener_id: Option<String>,
    pub browser_context_id: Option<String>,
}
