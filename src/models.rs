use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShizukuStatus {
  pub service_available: bool,
  pub pre_v11: bool,
  pub permission_granted: bool,
  pub should_show_request_rationale: bool,
  pub can_request_permission: bool,
  pub server_uid: Option<i32>,
  pub server_version: Option<i32>,
  pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermissionRequest {
  pub request_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermissionResponse {
  pub requested: bool,
  pub granted: bool,
  pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenShizukuResponse {
  pub opened: bool,
  pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAdbCommandRequest {
  pub command: String,
  pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAdbCommandResponse {
  pub exit_code: i32,
  pub stdout: String,
  pub stderr: String,
  pub duration_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSystemPropertyRequest {
  pub key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSystemPropertyResponse {
  pub key: String,
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPackagesRequest {
  pub include_system: Option<bool>,
  pub limit: Option<usize>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPackagesResponse {
  pub packages: Vec<String>,
}
