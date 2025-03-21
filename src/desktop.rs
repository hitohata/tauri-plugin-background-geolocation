use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<BackgroundGeolocation<R>> {
  Ok(BackgroundGeolocation(app.clone()))
}

/// Access to the background-geolocation APIs.
pub struct BackgroundGeolocation<R: Runtime>(AppHandle<R>);

impl<R: Runtime> BackgroundGeolocation<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
