use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::near_bindgen;

struct Writing {
  pub text: String,
  pub sender: String,
  pub receiver: String,
}

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub writing_list: Vec<Writing>, //This line gives err
}

#[near_bindgen]
impl Contract {}
