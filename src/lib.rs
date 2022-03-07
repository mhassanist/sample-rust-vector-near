//نحتاج استيراد التالي حتى يمكن لبيئة العمل ترجمة السطور التالية و تحويلها لكود يعمل على البلوك تشين
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
  message: String,
}
#[near_bindgen]
//ماكرو حتى يمكن لبيئة العمل ترجمة الكود التالي الى كود يعمل على البلوك تشين
#[near_bindgen]
impl Contract {
  //دالة لتغيير قيمة المتغير
  //message
  pub fn set_message(&mut self, msg: String) -> String {
    self.message = msg;
    return "updated".to_string();
  }

  //دالة لإرجاع قيمة المتغير
  //message
  pub fn get_message(&self) -> String {
    return self.message.to_string();
  }

  //دالة لفحص رصيد ههذا الحساب الموجود عليه الكود
  //ملحوظة: تحتاج
  //call
  // و ليس view
  pub fn get_balance() -> u128 {
    //env
    //مثل
    //Context
    //فى الأسمبلي سكريبت
    //و هو وسيلتنا للوصول الى معلومات من البلوك تشين نفسه
    //مثل أرصدة الحسابات وقيم الأموال المرسلة مع الاستعدعاءات و خلافه
    return env::account_balance();
  }
}
