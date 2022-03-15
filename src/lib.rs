use std::fmt::Write;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{near_bindgen, IntoStorageKey};
near_sdk::setup_alloc!();

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Writing {
  pub text: String,
  pub sender: String,
  pub receiver: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub writing_list: Vector<Writing>, //This line gives err
}

impl Default for Contract {
  fn default() -> Self {
    return Default::default();
  }
}

#[near_bindgen]
impl Contract {
  pub fn add(&self) -> String {
    return "Added".to_string();
  }
  pub fn get_writings(&self) -> u64 {
    return self.writing_list.len();
  }
}
