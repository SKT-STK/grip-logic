use std::{sync::Arc, thread, time::Duration};
use acc_shared_memory_rs::{ACCError, ACCSharedMemory};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use crate::{global::{AccBinds, BB_OFFSETS, Item, get_acc, get_acc_binds, get_acc_points}, input_simulator::bindings::{InputSimSoftware, InputSimulator}, utils::{ThreadState, WorkerControl, js_code_to_windows_vk}};
use tokio;

pub fn reinit_shared_memory() {
  thread::spawn(move || {
    let acc = loop {
      match ACCSharedMemory::new() {
        Ok(mem) => break mem,
        Err(ACCError::SharedMemoryNotAvailable) => {
          println!("Waiting for ACC to start...");
          thread::sleep(Duration::from_secs(1));
        },
        Err(err) => panic!("Unexpected error connecting to ACC: {:?}", err)
      }
    };
  
    let mut guard = match get_acc().write() {
      Ok(g) => g,
      Err(poisoned) => poisoned.into_inner()
    };
    *guard = Some(acc);
  });
}

pub async fn update_acc_points(app_handle: AppHandle) {
  let store = tokio::task::spawn_blocking(move || {
    app_handle.store("data.json")
  }).await.unwrap().unwrap();

  if let Some(tracks_cars) = store.get("tracks-cars") {
    let items: Vec<Item<String>> = serde_json::from_value(tracks_cars).unwrap();
    
    let mut guard = match get_acc_points().write() {
      Ok(g) => g,
      Err(poisoned) => poisoned.into_inner()
    };
    *guard = Some(items.into_iter().map(Into::into).collect());
  }
}

fn update_acc_binds(app_handle: &AppHandle) {
  let store = app_handle.store("settings.json").unwrap();
  if let (Some(some_binds), Some(some_kb_soft)) = (store.get("binds"), store.get("keyboard-software")) {
    let binds: Vec<Option<String>> = serde_json::from_value(some_binds).unwrap();
    let kb_soft: Option<String> = serde_json::from_value(some_kb_soft).unwrap();

    let mut binds = binds.iter().map(|v| {
      if let Some(value) = v {
        js_code_to_windows_vk(value)
      }
      else {
        None
      }
    }).collect::<Vec<Option<u16>>>();

    let bb_dec = binds.pop().unwrap();
    let bb_inc = binds.pop().unwrap();

    let mut guard = match get_acc_binds().write() {
      Ok(g) => g,
      Err(poisoned) => poisoned.into_inner()
    };
    *guard = Some(AccBinds {
      tcs: binds,
      bb_dec,
      bb_inc,
      kb_soft: match kb_soft.as_deref() {
        Some("ghub") => InputSimSoftware::LogitechGHubNew,
        Some("razer") => InputSimSoftware::RazerSynapse,
        _ => InputSimSoftware::default()
      }
    });
  }
}

pub fn start(control: Arc<WorkerControl>, app_handle: AppHandle) {
  reinit_shared_memory();
  update_acc_binds(&app_handle);

  let input_sim = InputSimulator::new();
  {
    let guard = match get_acc_binds().read() {
      Ok(g) => g,
      Err(poisoned) => poisoned.into_inner()
    };
    input_sim.init(guard.as_ref().unwrap_or(&AccBinds::default()).kb_soft, 0);
  }

  let duration_1ms = Duration::from_millis(1);

  loop {
    {
      let mut state = control.state.lock().unwrap();
      while *state == ThreadState::Paused {
        state = control.wake.wait(state).unwrap();
      }
    }

    update_acc_binds(&app_handle);
    {
      let guard = match get_acc_binds().read() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner()
      };
      input_sim.reinit(guard.as_ref().unwrap_or(&AccBinds::default()).kb_soft, 0);
    }

    loop {
      thread::sleep(Duration::from_millis(33));

      {
        let state = control.state.lock().unwrap();
        match *state {
          ThreadState::Running => {},
          ThreadState::Paused => break
        }
      }

      let mut acc_guard = match get_acc().write() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner()
      };
      let data_option = if let Some(ref mut acc) = *acc_guard {
        match acc.read_shared_memory() {
          Ok(data) => data,
          Err(_) => {
            *acc_guard = None;
            reinit_shared_memory();
            None
          }
        }
      } else { None };

      if let Some(data) = data_option {
        let curr_track_perc = (data.graphics.normalized_car_position * 1000f32) as i32;
        let curr_speed = data.physics.speed_kmh as i32;
        let curr_track = data.statics.track;
        let curr_car = data.statics.car_model;
        let curr_bb = data.physics.brake_bias;

        let points_guard = match get_acc_points().read() {
          Ok(g) => g,
          Err(poisoned) => poisoned.into_inner()
        };
        if let Some(points) = points_guard.as_ref() {
          let binds_guard = match get_acc_binds().read() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner()
          };
          let binds = binds_guard.as_ref().unwrap();

          for item in points {
            if curr_track != item.track.name || curr_car != item.track.car.name { continue; }
            if let Some(entries) = item.track.car.entries.get(&curr_track_perc) {
              for entry in entries {
                if curr_speed < entry.minSpeed { continue; }
                if entry.method == "TC" && entry.value.is_some() {
                  let bind_option = binds.tcs[entry.value.unwrap() as usize];
                  if let Some(bind) = bind_option {
                    input_sim.key_down(bind);
                    std::thread::sleep(duration_1ms);
                    input_sim.key_up(bind);
                  }
                }
                else if entry.method == "BB" && entry.value.is_some() {
                  let curr_bb = curr_bb + (BB_OFFSETS.get(&curr_car).unwrap_or(&0f32) / 100f32);
                  let curr_bb = (curr_bb * 1000f32) as u32;
                  let curr_bb = if curr_bb % 2 == 1 { curr_bb + 1 } else { curr_bb };
                  let bb_diff = curr_bb as i32 - (entry.value.unwrap() * 10f32) as i32;
                  let bb_diff = bb_diff / 2;
                  let bind_option = if bb_diff < 0 { binds.bb_inc } else { binds.bb_dec };
                  let bb_diff = bb_diff.abs();
                  if let Some(bind) = bind_option {
                    for _ in 0..bb_diff {
                      input_sim.key_down(bind);
                      thread::sleep(duration_1ms);
                      input_sim.key_up(bind);
                      thread::sleep(duration_1ms);
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
