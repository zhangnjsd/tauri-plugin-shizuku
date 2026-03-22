use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Shizuku<R>> {
  Ok(Shizuku(app.clone()))
}

/// Access to the shizuku APIs.
pub struct Shizuku<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Shizuku<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }

  pub fn get_status(&self) -> crate::Result<ShizukuStatus> {
    Err(crate::Error::UnsupportedPlatform(
      "Shizuku is only available on Android",
    ))
  }

  pub fn request_permission(
    &self,
    _payload: RequestPermissionRequest,
  ) -> crate::Result<RequestPermissionResponse> {
    Err(crate::Error::UnsupportedPlatform(
      "Shizuku is only available on Android",
    ))
  }

  pub fn open_shizuku(&self) -> crate::Result<OpenShizukuResponse> {
    Err(crate::Error::UnsupportedPlatform(
      "Shizuku is only available on Android",
    ))
  }

  pub fn run_adb_command(
    &self,
    _payload: RunAdbCommandRequest,
  ) -> crate::Result<RunAdbCommandResponse> {
    Err(crate::Error::UnsupportedPlatform(
      "Shizuku is only available on Android",
    ))
  }

  pub fn get_system_property(
    &self,
    _payload: GetSystemPropertyRequest,
  ) -> crate::Result<GetSystemPropertyResponse> {
    Err(crate::Error::UnsupportedPlatform(
      "Shizuku is only available on Android",
    ))
  }

  pub fn list_packages(&self, _payload: ListPackagesRequest) -> crate::Result<ListPackagesResponse> {
    Err(crate::Error::UnsupportedPlatform(
      "Shizuku is only available on Android",
    ))
  }
}
