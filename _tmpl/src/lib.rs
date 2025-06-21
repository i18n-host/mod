#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use aok::{Result, OK};

pub enum SignUp {}

#[iat::captcha]
pub async fn signup() -> Result<()> {
  OK
}
