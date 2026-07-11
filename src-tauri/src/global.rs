use std::{collections::HashMap, hash::{BuildHasherDefault, DefaultHasher}, sync::{OnceLock, RwLock}};
use acc_shared_memory_rs::ACCSharedMemory;
use phf::phf_map;

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

impl<T> Car<T>
where
  T: ValidEntryVecHashMapKeyType,
  T: Eq,
  T: std::hash::Hash,
  T: 'static
{
  pub fn parse_keys_of_entries(self) -> Car<i32> {
    let parsed_entries = self.entries.into_iter().map(|(key, value)| {
      let any_key = &key as &dyn std::any::Any;
      let new_key = if let Some(string_key) = any_key.downcast_ref::<String>() {
        (string_key.parse::<f32>().unwrap() * 10f32) as i32
      }
      else {
        *(any_key.downcast_ref::<i32>().unwrap())
      };

      (new_key, value)
    }).collect();

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

#[derive(serde::Deserialize, Debug)]
pub struct Item<T = i32>
where
  T: ValidEntryVecHashMapKeyType,
  T: Eq,
  T: std::hash::Hash
{
  pub track: Track<T>
}

static GLOBAL_ACC: OnceLock<RwLock<Option<ACCSharedMemory>>> = OnceLock::new();
pub static BB_OFFSETS: phf::Map<&'static str, i32> = phf_map! {
  "amr_vantage_v12_gt3" => -7,
  "audi_r8_lms" => -14,
  "bentley_continental_gt3" => -7,
  "bmw_m6_gt3" => -15,
  "emil_frey_jaguar_g3" => -7,
  "ferrari_488_gt3" => -17,
  "honda_nsx_gt3" => -14,
  "lamborghini_gallardo_g3_reiter" => -14,
  "lamborghini_huracan_gt3" => -14,
  "lamborghini_huracan_st" => -14,
  "lexus_rcf_gt3" => -14,
  "mclaren_650s_gt3" => -17,
  "mercedes_amg_gt3" => -14,
  "nissan_gtr_nismo_gt3" => -15,
  "porsche_991_gt3_r" => -21,
  "porsche_991_ii_gt3_cup" => -5,
  "amr_v8_vantage_gt3" => -7,
  "audi_r8_lms_evo" => -14,
  "honda_nsx_gt3_evo" => -14,
  "lamborghini_huracan_gt3_evo" => -14,
  "mclaren_720s_gt3" => -17,
  "porsche_911_ii_gt3_r" => -21,
  "alpine_a110_gt4" => -15,
  "amr_vantage_amr_gt4" => -20,
  "audi_r8_lms_gt4" => -15,
  "bmw_m4_gt4" => -22,
  "chevrolet_camaro_gt4_r" => -18,
  "ginetta_g55_gt4" => -18,
  "ktm_xbow_gt4" => -20,
  "maserati_gran_turismo_mc_gt4" => -15,
  "mclaren_570s_gt4" => -9,
  "mercedes_amg_gt4" => -20,
  "porsche_718_cayman_gt4_mr" => -20,
  "ferrari_488_gt3_evo" => -17,
  "mercedes_amg_gt3_evo" => -14,
  "bmw_m4_gt3" => -14,
  "audi_r8_lms_evo_ii" => -14,
  "bmw_m2_cup" => -17,
  "ferrari_488_challenge_evo" => -13,
  "lamborghini_huracan_st_evo2" => -14,
  "porsche_992_gt3_cup" => -5,
  "ferrari_296_gt3" => -5,
  "amr_vantage_v12_gt3_2013" => -7,
  "audi_r8_lms_2015" => -14,
  "bentley_continental_gt3_2015" => -7,
  "bentley_continental_gt3_2018" => -7,
  "bmw_m6_gt3_2017" => -15,
  "emil_frey_jaguar_g3_2012" => -7,
  "ferrari_488_gt3_2018" => -17,
  "honda_nsx_gt3_2017" => -14,
  "lamborghini_gallardo_g3_reiter_2017" => -14,
  "lamborghini_huracan_gt3_2015" => -14,
  "lamborghini_huracan_st_2015" => -14,
  "lexus_rcf_gt3_2016" => -14,
  "mclaren_650s_gt3_2015" => -17,
  "mercedes_amg_gt3_2015" => -14,
  "nissan_gtr_nismo_gt3_2015" => -15,
  "nissan_gtr_nismo_gt3_2018" => -15,
  "porsche_991_gt3_r_2018" => -21,
  "porsche_991_ii_gt3_cup_2017" => -5,
  "amr_v8_vantage_gt3_2019" => -7,
  "audi_r8_lms_evo_2019" => -14,
  "honda_nsx_gt3_evo_2019" => -14,
  "lamborghini_huracan_gt3_evo_2019" => -14,
  "mclaren_720s_gt3_2019" => -17,
  "porsche_911_ii_gt3_r_2019" => -21,
  "alpine_a110_gt4_2018" => -15,
  "amr_vantage_amr_gt4_2018" => -20,
  "audi_r8_lms_gt4_2016" => -15,
  "bmw_m4_gt4_2018" => -22,
  "chevrolet_camaro_gt4_r_2017" => -18,
  "ginetta_g55_gt4_2012" => -18,
  "ktm_xbow_gt4_2016" => -20,
  "maserati_gran_turismo_mc_gt4_2016" => -15,
  "mclaren_570s_gt4_2016" => -9,
  "mercedes_amg_gt4_2016" => -20,
  "porsche_718_cayman_gt4_mr_2019" => -20,
  "ferrari_488_gt3_evo_2020" => -17,
  "mercedes_amg_gt3_evo_2020" => -14,
  "audi_r8_lms_evo_ii_2022" => -14,
  "bmw_m2_cup_2020" => -17,
  "ferrari_488_challenge_evo_2020" => -13,
  "lamborghini_huracan_st_evo2_2021" => -14,
  "porsche_992_gt3_cup_2021" => -5,
  "ferrari_296_gt3_2025" => -5
};
static ACC_POINTS: OnceLock<RwLock<Option<Vec<Item>>>> = OnceLock::new();

pub fn get_acc() -> &'static RwLock<Option<ACCSharedMemory>> {
  GLOBAL_ACC.get_or_init(|| RwLock::new(None))
}

pub fn get_acc_points() -> &'static RwLock<Option<Vec<Item>>> {
  ACC_POINTS.get_or_init(|| RwLock::new(None))
}
