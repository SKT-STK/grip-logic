use std::sync::{Condvar, Mutex};

#[derive(PartialEq)]
pub enum ThreadState {
  Running,
  Paused
}

pub struct WorkerControl {
  pub state: Mutex<ThreadState>,
  pub wake: Condvar
}

impl WorkerControl {
  pub fn new() -> Self {
    Self {
      state: Mutex::new(ThreadState::Paused),
      wake: Condvar::new()
    }
  }

  pub fn resume(&self) {
    let mut state = self.state.lock().unwrap();
    *state = ThreadState::Running;

    self.wake.notify_one();
  }

  pub fn pause(&self) {
    let mut state = self.state.lock().unwrap();
    *state = ThreadState::Paused;
  }
}

pub fn js_code_to_windows_vk(js_code: &str) -> Option<u16> {
  match js_code {
    // Numpad Keys
    "Numpad0" => Some(0x60), // VK_NUMPAD0
    "Numpad1" => Some(0x61), // VK_NUMPAD1
    "Numpad2" => Some(0x62), // VK_NUMPAD2
    "Numpad3" => Some(0x63), // VK_NUMPAD3
    "Numpad4" => Some(0x64), // VK_NUMPAD4
    "Numpad5" => Some(0x65), // VK_NUMPAD5
    "Numpad6" => Some(0x66), // VK_NUMPAD6
    "Numpad7" => Some(0x67), // VK_NUMPAD7
    "Numpad8" => Some(0x68), // VK_NUMPAD8
    "Numpad9" => Some(0x69), // VK_NUMPAD9
    "NumpadMultiply" => Some(0x6A), // VK_MULTIPLY
    "NumpadAdd"      => Some(0x6B), // VK_ADD
    "NumpadSubtract" => Some(0x6D), // VK_SUBTRACT
    "NumpadDecimal"  => Some(0x6E), // VK_DECIMAL
    "NumpadDivide"   => Some(0x6F), // VK_DIVIDE

    // Standard Alphanumeric (A-Z, 0-9)
    "KeyA" => Some(0x41), "KeyB" => Some(0x42), "KeyC" => Some(0x43),
    "KeyD" => Some(0x44), "KeyE" => Some(0x45), "KeyF" => Some(0x46),
    "KeyG" => Some(0x47), "KeyH" => Some(0x48), "KeyI" => Some(0x49),
    "KeyJ" => Some(0x4A), "KeyK" => Some(0x4B), "KeyL" => Some(0x4C),
    "KeyM" => Some(0x4D), "KeyN" => Some(0x4E), "KeyO" => Some(0x4F),
    "KeyP" => Some(0x50), "KeyQ" => Some(0x51), "KeyR" => Some(0x52),
    "KeyS" => Some(0x53), "KeyT" => Some(0x54), "KeyU" => Some(0x55),
    "KeyV" => Some(0x56), "KeyW" => Some(0x57), "KeyX" => Some(0x58),
    "KeyY" => Some(0x59), "KeyZ" => Some(0x5A),
    
    "Digit0" => Some(0x30), "Digit1" => Some(0x31), "Digit2" => Some(0x32),
    "Digit3" => Some(0x33), "Digit4" => Some(0x34), "Digit5" => Some(0x35),
    "Digit6" => Some(0x36), "Digit7" => Some(0x37), "Digit8" => Some(0x38),
    "Digit9" => Some(0x39),

    // Function Keys
    "F1" => Some(0x70), "F2" => Some(0x71), "F3" => Some(0x72),
    "F4" => Some(0x73), "F5" => Some(0x74), "F6" => Some(0x75),
    "F7" => Some(0x76), "F8" => Some(0x77), "F9" => Some(0x78),
    "F10" => Some(0x79), "F11" => Some(0x7A), "F12" => Some(0x7B),

    // Controls & Navigation
    "Escape"    => Some(0x1B), // VK_ESCAPE
    "Space"     => Some(0x20), // VK_SPACE
    "Enter"     => Some(0x0D), // VK_RETURN
    "Backspace" => Some(0x08), // VK_BACK
    "Tab"       => Some(0x09), // VK_TAB
    
    "ArrowLeft"  => Some(0x25), // VK_LEFT
    "ArrowUp"    => Some(0x26), // VK_UP
    "ArrowRight" => Some(0x27), // VK_RIGHT
    "ArrowDown"  => Some(0x28), // VK_DOWN

      _ => None,
  }
}
