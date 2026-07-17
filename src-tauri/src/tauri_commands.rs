use std::sync::Arc;
use std::sync::OnceLock;
use std::thread;
use tauri::AppHandle;
use tauri::Window;
use tokio::runtime::Builder;
use crate::acc;
use crate::acc_data;
use crate::global;
use crate::utils::WorkerControl;

static mut ONCE_INIT: bool = false;
static WORKER_CONTROL: OnceLock<Arc<WorkerControl>> = OnceLock::new();

fn control() -> &'static Arc<WorkerControl> {
  WORKER_CONTROL.get_or_init(|| {
    Arc::new(WorkerControl::new())
  })
}

#[tauri::command]
pub fn set_window_visible(win: Window, app_handle: AppHandle) {
  unsafe {
    if ONCE_INIT { return; }
    ONCE_INIT = true;
  };

  win.show().expect("failed to show main window");

  let worker_control = Arc::clone(control());
  thread::spawn(move || {
    let rt = Builder::new_current_thread()
      .enable_all()
      .build()
      .unwrap();
    rt.block_on(async {
      acc::update_acc_points(app_handle.clone()).await;
    });
    drop(rt);
    acc::start(worker_control, app_handle);
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
pub fn fetch_bb_offset(car_name: String) -> Option<f32> {
  global::BB_OFFSETS.get(&car_name).copied()
}

#[tauri::command]
pub async fn data_store_saved(app_handle: AppHandle) {
  acc::update_acc_points(app_handle).await;
}

#[tauri::command]
pub fn resume_main() {
  control().resume();
}

#[tauri::command]
pub fn pause_main() {
  control().pause();
}
