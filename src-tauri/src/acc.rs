use std::{collections::HashMap, thread, time::Duration};
use acc_shared_memory_rs::{ACCError, ACCSharedMemory};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use crate::global::{Car, Item, Track, get_acc, get_acc_points};
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
    let items = items.into_iter().map(|item| {
      Item {
        track: Track {
          name: item.track.name,
          car: item.track.car.parse_keys_of_entries()
        }
      }
    }).collect::<Vec<Item<i32>>>();
    

    let mut guard = match get_acc_points().write() {
      Ok(g) => g,
      Err(poisoned) => poisoned.into_inner()
    };
    *guard = Some(items);

    println!("{:?}", *guard);
  }
}

pub fn start() {
  reinit_shared_memory();

  let duration = Duration::from_millis(33);
  loop {

    thread::sleep(duration);
  }
}
