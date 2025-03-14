use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserPermissions {
    pub resources: Vec<&'static str>,
    pub from: &'static str,
    pub version: &'static str,
    pub username: Option<Arc<String>>,
}
