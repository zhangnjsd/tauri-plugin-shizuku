use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "app.tauri.shizuku";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_shizuku);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Shizuku<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "ShizukuPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_shizuku)?;
  Ok(Shizuku(handle))
}

/// Access to the shizuku APIs.
pub struct Shizuku<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Shizuku<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }

  pub fn get_status(&self) -> crate::Result<ShizukuStatus> {
    self
      .0
      .run_mobile_plugin("getStatus", ())
      .map_err(Into::into)
  }

  pub fn request_permission(
    &self,
    payload: RequestPermissionRequest,
  ) -> crate::Result<RequestPermissionResponse> {
    self
      .0
      .run_mobile_plugin("requestPermission", payload)
      .map_err(Into::into)
  }

  pub fn open_shizuku(&self) -> crate::Result<OpenShizukuResponse> {
    self
      .0
      .run_mobile_plugin("openShizuku", ())
      .map_err(Into::into)
  }

  pub fn run_adb_command(
    &self,
    payload: RunAdbCommandRequest,
  ) -> crate::Result<RunAdbCommandResponse> {
    self
      .0
      .run_mobile_plugin("runAdbCommand", payload)
      .map_err(Into::into)
  }

  pub fn get_system_property(
    &self,
    payload: GetSystemPropertyRequest,
  ) -> crate::Result<GetSystemPropertyResponse> {
    self
      .0
      .run_mobile_plugin("getSystemProperty", payload)
      .map_err(Into::into)
  }

  pub fn list_packages(&self, payload: ListPackagesRequest) -> crate::Result<ListPackagesResponse> {
    self
      .0
      .run_mobile_plugin("listPackages", payload)
      .map_err(Into::into)
  }
}
