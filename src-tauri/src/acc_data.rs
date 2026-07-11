use crate::{acc::reinit_shared_memory, global::get_acc};

pub fn get_car_and_track_name() -> Option<Vec<String>> {
  let mut guard = match get_acc().try_write() {
    Ok(g) => g,
    Err(_) => return None
  };

  if let Some(ref mut acc) = *guard {
    match acc.read_shared_memory() {
      Ok(Some(data)) => return Some(vec![data.statics.car_model, data.statics.track]),
      Ok(None) => return None,
      Err(_) => {
        *guard = None;
        reinit_shared_memory();
        return None;
      }
    }
  }
  else {
    None
  }
}

pub fn get_curr_track_perc() -> Option<f32> {
  let mut guard = match get_acc().try_write() {
    Ok(g) => g,
    Err(_) => return None
  };

  if let Some(ref mut acc) = *guard {
    match acc.read_shared_memory() {
      Ok(Some(data)) => return Some(data.graphics.normalized_car_position),
      Ok(None) => return None,
      Err(_) => {
        *guard = None;
        reinit_shared_memory();
        return None;
      }
    }
  }
  else {
    None
  }
}
