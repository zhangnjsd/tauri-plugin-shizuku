use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Shizuku;
#[cfg(mobile)]
use mobile::Shizuku;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the shizuku APIs.
pub trait ShizukuExt<R: Runtime> {
  fn shizuku(&self) -> &Shizuku<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ShizukuExt<R> for T {
  fn shizuku(&self) -> &Shizuku<R> {
    self.state::<Shizuku<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("shizuku")
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::get_status,
      commands::request_permission,
      commands::open_shizuku,
      commands::run_adb_command,
      commands::get_system_property,
      commands::list_packages,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let shizuku = mobile::init(app, api)?;
      #[cfg(desktop)]
      let shizuku = desktop::init(app, api)?;
      app.manage(shizuku);
      Ok(())
    })
    .build()
}
