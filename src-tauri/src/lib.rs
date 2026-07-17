mod tauri_commands;
mod acc_data;
mod global;
mod acc;
mod input_simulator;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      tauri_commands::set_window_visible,
      tauri_commands::get_car_and_track_name,
      tauri_commands::get_curr_track_perc,
      tauri_commands::fetch_bb_offset,
      tauri_commands::data_store_saved,
      tauri_commands::resume_main,
      tauri_commands::pause_main
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
