use std::{ffi::c_void, ptr::null_mut};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum InputSimSoftware {
  RazerSynapse = 3,
  LogitechGHubNew = 6
}

impl Default for InputSimSoftware {
  fn default() -> Self {
    Self::LogitechGHubNew
  }
}

pub struct InputSimulator {
  init: unsafe extern "C" fn(u32, u32, *mut c_void) -> u32,
  destroy: unsafe extern "C" fn(),
  key_down: unsafe extern "C" fn(u16) -> i32,
  key_up: unsafe extern "C" fn(u16) -> i32,
  _lib: libloading::Library
}

impl InputSimulator {
  pub fn new() -> Self {
    let lib = unsafe { libloading::Library::new("InputSimulator.dll") }.unwrap();
    let init_sym = unsafe { lib.get(b"IbSendInit") }.unwrap();
    let destroy_sym = unsafe { lib.get(b"IbSendDestroy") }.unwrap();
    let key_down_sym = unsafe { lib.get(b"IbSendKeybdDown") }.unwrap();
    let key_up_sym = unsafe { lib.get(b"IbSendKeybdUp") }.unwrap();

    let init = *init_sym;
    let destroy = *destroy_sym;
    let key_down = *key_down_sym;
    let key_up = *key_up_sym;

    Self {
      _lib: lib,
      init,
      destroy,
      key_down,
      key_up
    }
  }

  pub fn init(&self, software: InputSimSoftware, connection_id: u32) {
    unsafe { (self.init)(software as u32, connection_id, null_mut()); };
  }

  pub fn key_down(&self, bind: u16) {
    unsafe { (self.key_down)(bind); };
  }

  pub fn key_up(&self, bind: u16) {
    unsafe { (self.key_up)(bind); };
  }

  pub fn reinit(&self, software: InputSimSoftware, connection_id: u32) {
    unsafe { (self.destroy)(); };
    self.init(software, connection_id);
  }
}

impl Drop for InputSimulator {
  fn drop(&mut self) {
    unsafe { (self.destroy)(); };
  }
}
