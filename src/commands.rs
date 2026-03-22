use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ShizukuExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.shizuku().ping(payload)
}

#[command]
pub(crate) async fn get_status<R: Runtime>(app: AppHandle<R>) -> Result<ShizukuStatus> {
    app.shizuku().get_status()
}

#[command]
pub(crate) async fn request_permission<R: Runtime>(
    app: AppHandle<R>,
    payload: Option<RequestPermissionRequest>,
) -> Result<RequestPermissionResponse> {
    app
        .shizuku()
        .request_permission(payload.unwrap_or_default())
}

#[command]
pub(crate) async fn open_shizuku<R: Runtime>(app: AppHandle<R>) -> Result<OpenShizukuResponse> {
    app.shizuku().open_shizuku()
}

#[command]
pub(crate) async fn run_adb_command<R: Runtime>(
    app: AppHandle<R>,
    payload: RunAdbCommandRequest,
) -> Result<RunAdbCommandResponse> {
    app.shizuku().run_adb_command(payload)
}

#[command]
pub(crate) async fn get_system_property<R: Runtime>(
    app: AppHandle<R>,
    payload: GetSystemPropertyRequest,
) -> Result<GetSystemPropertyResponse> {
    app.shizuku().get_system_property(payload)
}

#[command]
pub(crate) async fn list_packages<R: Runtime>(
    app: AppHandle<R>,
    payload: Option<ListPackagesRequest>,
) -> Result<ListPackagesResponse> {
    app.shizuku().list_packages(payload.unwrap_or_default())
}
