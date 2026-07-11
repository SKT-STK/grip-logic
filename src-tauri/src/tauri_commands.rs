use std::thread;
use tauri::AppHandle;
use tauri::Window;
use crate::acc;
use crate::acc_data;
use crate::global;

#[tauri::command]
pub fn set_window_visible(win: Window) {
  win.show().expect("failed to show main window");

  thread::spawn(move || {
    acc::start();
  });
}

#[tauri::command]
pub fn get_car_and_track_name() -> Option<Vec<String>> {
  acc_data::get_car_and_track_name()
}

#[tauri::command]
pub fn get_curr_track_perc() -> Option<f32> {
  acc_data::get_curr_track_perc()
}

#[tauri::command]
pub fn fetch_bb_offset(car_name: String) -> Option<i32> {
  global::BB_OFFSETS.get(&car_name).copied()
}

#[tauri::command]
pub async fn data_store_saved(app_handle: AppHandle) {
  acc::update_acc_points(app_handle).await;
}
