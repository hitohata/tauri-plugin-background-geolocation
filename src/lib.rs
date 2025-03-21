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
use desktop::BackgroundGeolocation;
#[cfg(mobile)]
use mobile::BackgroundGeolocation;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the background-geolocation APIs.
pub trait BackgroundGeolocationExt<R: Runtime> {
  fn background_geolocation(&self) -> &BackgroundGeolocation<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BackgroundGeolocationExt<R> for T {
  fn background_geolocation(&self) -> &BackgroundGeolocation<R> {
    self.state::<BackgroundGeolocation<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("background-geolocation")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let background_geolocation = mobile::init(app, api)?;
      #[cfg(desktop)]
      let background_geolocation = desktop::init(app, api)?;
      app.manage(background_geolocation);
      Ok(())
    })
    .build()
}
