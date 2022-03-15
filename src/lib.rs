use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::near_bindgen;
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
  pub fn add(&mut self) -> String {
    let w = Writing {
      text: "val".to_string(),
      sender: "val".to_string(),
      receiver: "val".to_string(),
    };

    let _ = &self.writing_list.push(&w);

    return "Added".to_string();
  }
  pub fn get_writings(&self) -> u64 {
    return self.writing_list.len();
  }
}
