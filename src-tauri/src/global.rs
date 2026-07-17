use std::{collections::HashMap, hash::{BuildHasherDefault, DefaultHasher}, sync::{OnceLock, RwLock}};
use acc_shared_memory_rs::ACCSharedMemory;
use phf::phf_map;

use crate::input_simulator::bindings::InputSimSoftware;

pub trait ValidEntryVecHashMapKeyType {}
impl ValidEntryVecHashMapKeyType for i32 {}
impl ValidEntryVecHashMapKeyType for String {}

#[derive(serde::Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct EntryVec {
  pub method: String,
  pub minSpeed: i32,
  pub value: Option<f32>
}

#[derive(serde::Deserialize, Debug)]
pub struct Car<T = i32>
where
  T: ValidEntryVecHashMapKeyType,
  T: Eq,
  T: std::hash::Hash
{
  pub name: String,
  pub entries: HashMap<T, Vec<EntryVec>, BuildHasherDefault<DefaultHasher>>
}

impl Into<Car<i32>> for Car<String> {
  fn into(self) -> Car<i32> {
    let parsed_entries = self
      .entries
      .into_iter()
      .map(|(k, v)| ((k.parse::<f32>().unwrap() * 10f32) as i32, v))
      .collect();
    Car {
      name: self.name,
      entries: parsed_entries
    }
  }
}

#[derive(serde::Deserialize, Debug)]
pub struct Track<T = i32>
where
  T: ValidEntryVecHashMapKeyType,
  T: Eq,
  T: std::hash::Hash
{
  pub name: String,
  pub car: Car<T>
}

impl Into<Track<i32>> for Track<String> {
  fn into(self) -> Track<i32> {
    Track {
      name: self.name,
      car: self.car.into()
    }
  }
}

#[derive(serde::Deserialize, Debug)]
pub struct Item<T = i32>
where
  T: ValidEntryVecHashMapKeyType,
  T: Eq,
  T: std::hash::Hash
{
  pub track: Track<T>
}

impl Into<Item<i32>> for Item<String> {
  fn into(self) -> Item<i32> {
    Item {
      track: self.track.into()
    }
  }
}

pub struct AccBinds {
  pub tcs: Vec<Option<u16>>,
  pub bb_dec: Option<u16>,
  pub bb_inc: Option<u16>,
  pub kb_soft: InputSimSoftware
}

static GLOBAL_ACC: OnceLock<RwLock<Option<ACCSharedMemory>>> = OnceLock::new();
pub static BB_OFFSETS: phf::Map<&'static str, f32> = phf_map! {
  "amr_vantage_v12_gt3" => -7f32,
  "audi_r8_lms" => -14f32,
  "bentley_continental_gt3" => -7f32,
  "bmw_m6_gt3" => -15f32,
  "emil_frey_jaguar_g3" => -7f32,
  "ferrari_488_gt3" => -17f32,
  "honda_nsx_gt3" => -14f32,
  "lamborghini_gallardo_g3_reiter" => -14f32,
  "lamborghini_huracan_gt3" => -14f32,
  "lamborghini_huracan_st" => -14f32,
  "lexus_rcf_gt3" => -14f32,
  "mclaren_650s_gt3" => -17f32,
  "mercedes_amg_gt3" => -14f32,
  "nissan_gtr_nismo_gt3" => -15f32,
  "porsche_991_gt3_r" => -21f32,
  "porsche_991_ii_gt3_cup" => -5f32,
  "amr_v8_vantage_gt3" => -7f32,
  "audi_r8_lms_evo" => -14f32,
  "honda_nsx_gt3_evo" => -14f32,
  "lamborghini_huracan_gt3_evo" => -14f32,
  "mclaren_720s_gt3" => -17f32,
  "porsche_911_ii_gt3_r" => -21f32,
  "alpine_a110_gt4" => -15f32,
  "amr_vantage_amr_gt4" => -20f32,
  "audi_r8_lms_gt4" => -15f32,
  "bmw_m4_gt4" => -22f32,
  "chevrolet_camaro_gt4_r" => -18f32,
  "ginetta_g55_gt4" => -18f32,
  "ktm_xbow_gt4" => -20f32,
  "maserati_gran_turismo_mc_gt4" => -15f32,
  "mclaren_570s_gt4" => -9f32,
  "mercedes_amg_gt4" => -20f32,
  "porsche_718_cayman_gt4_mr" => -20f32,
  "ferrari_488_gt3_evo" => -17f32,
  "mercedes_amg_gt3_evo" => -14f32,
  "bmw_m4_gt3" => -14f32,
  "audi_r8_lms_evo_ii" => -14f32,
  "bmw_m2_cup" => -17f32,
  "ferrari_488_challenge_evo" => -13f32,
  "lamborghini_huracan_st_evo2" => -14f32,
  "porsche_992_gt3_cup" => -5f32,
  "ferrari_296_gt3" => -5f32,
  "amr_vantage_v12_gt3_2013" => -7f32,
  "audi_r8_lms_2015" => -14f32,
  "bentley_continental_gt3_2015" => -7f32,
  "bentley_continental_gt3_2018" => -7f32,
  "bmw_m6_gt3_2017" => -15f32,
  "emil_frey_jaguar_g3_2012" => -7f32,
  "ferrari_488_gt3_2018" => -17f32,
  "honda_nsx_gt3_2017" => -14f32,
  "lamborghini_gallardo_g3_reiter_2017" => -14f32,
  "lamborghini_huracan_gt3_2015" => -14f32,
  "lamborghini_huracan_st_2015" => -14f32,
  "lexus_rcf_gt3_2016" => -14f32,
  "mclaren_650s_gt3_2015" => -17f32,
  "mercedes_amg_gt3_2015" => -14f32,
  "nissan_gtr_nismo_gt3_2015" => -15f32,
  "nissan_gtr_nismo_gt3_2018" => -15f32,
  "porsche_991_gt3_r_2018" => -21f32,
  "porsche_991_ii_gt3_cup_2017" => -5f32,
  "amr_v8_vantage_gt3_2019" => -7f32,
  "audi_r8_lms_evo_2019" => -14f32,
  "honda_nsx_gt3_evo_2019" => -14f32,
  "lamborghini_huracan_gt3_evo_2019" => -14f32,
  "mclaren_720s_gt3_2019" => -17f32,
  "porsche_911_ii_gt3_r_2019" => -21f32,
  "alpine_a110_gt4_2018" => -15f32,
  "amr_vantage_amr_gt4_2018" => -20f32,
  "audi_r8_lms_gt4_2016" => -15f32,
  "bmw_m4_gt4_2018" => -22f32,
  "chevrolet_camaro_gt4_r_2017" => -18f32,
  "ginetta_g55_gt4_2012" => -18f32,
  "ktm_xbow_gt4_2016" => -20f32,
  "maserati_gran_turismo_mc_gt4_2016" => -15f32,
  "mclaren_570s_gt4_2016" => -9f32,
  "mercedes_amg_gt4_2016" => -20f32,
  "porsche_718_cayman_gt4_mr_2019" => -20f32,
  "ferrari_488_gt3_evo_2020" => -17f32,
  "mercedes_amg_gt3_evo_2020" => -14f32,
  "audi_r8_lms_evo_ii_2022" => -14f32,
  "bmw_m2_cup_2020" => -17f32,
  "ferrari_488_challenge_evo_2020" => -13f32,
  "lamborghini_huracan_st_evo2_2021" => -14f32,
  "porsche_992_gt3_cup_2021" => -5f32,
  "ferrari_296_gt3_2025" => -5f32
};
static ACC_POINTS: OnceLock<RwLock<Option<Vec<Item>>>> = OnceLock::new();
static ACC_BINDS: OnceLock<RwLock<Option<AccBinds>>> = OnceLock::new();

pub fn get_acc() -> &'static RwLock<Option<ACCSharedMemory>> {
  GLOBAL_ACC.get_or_init(|| RwLock::new(None))
}

pub fn get_acc_points() -> &'static RwLock<Option<Vec<Item>>> {
  ACC_POINTS.get_or_init(|| RwLock::new(None))
}

pub fn get_acc_binds() -> &'static RwLock<Option<AccBinds>> {
  ACC_BINDS.get_or_init(|| RwLock::new(None))
}
