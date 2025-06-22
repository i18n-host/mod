pub(crate) mod db;
mod r;

pub mod err;
pub mod signin;
pub mod signup;

use serde::{Deserialize, Serialize};
use simple_useragent::UserAgentParser;

#[static_init::dynamic]
pub static UA: UserAgentParser = UserAgentParser::new();

#[derive(Deserialize, Serialize, Debug)]
pub struct BrowserMeta {
  pub ip: Option<std::net::IpAddr>,
  pub brand: String,
  pub ver: String,
  pub os: String,
  pub os_ver: String,
}

pub async fn test(
  timezone: i8,
  dpi: u8,
  w: u16,
  h: u16,
  os_ver: &str,
  arch: &str,
  cpu_num: u32,
  gpu: &str,
  headers: &http::HeaderMap,
) {
  let ua = UA.parse(
    headers
      .get("user-agent")
      .and_then(|v| v.to_str().ok())
      .unwrap_or_default(),
  );
  // dbg!(ua);
}
