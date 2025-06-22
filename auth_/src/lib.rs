pub(crate) mod db;
mod r;

pub mod err;
pub mod signin;
pub mod signup;

use serde::{Deserialize, Serialize};
use simple_useragent::UserAgentParser;

#[derive(Deserialize, Serialize, Debug)]
pub struct BrowserMeta {
  pub ip: Option<std::net::IpAddr>,
  pub brand: String,
  pub ver: String,
  pub os: String,
  pub os_ver: String,
}

#[static_init::dynamic]
pub static UA: UserAgentParser = UserAgentParser::new();

// uid: u64,
// ip: impl AsRef<[u8]>,
// timezone: i8,
// dpi: u8,
// w: u16,
// h: u16,
// os_ver: impl AsRef<str>,
// arch: impl AsRef<str>,
// cpu_num: u32,
// gpu: impl AsRef<str>,
// brand: impl AsRef<str>,
// os_name: impl AsRef<str>,
// browser_name: impl AsRef<str>,
// browser_ver: impl AsRef<str>,
// browser_lang: impl AsRef<str>,

pub fn test(
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

  dbg!(ua.client.family);
  dbg!(ua.client.version);

  let browser_lang = headers
    .get("accept-language")
    .and_then(|v| v.to_str().ok())
    .unwrap_or_default()
    .split(",")
    .next()
    .unwrap_or_default();

  dbg!((
    timezone,
    dpi,
    w,
    h,
    os_ver,
    arch,
    cpu_num,
    gpu,
    browser_lang
  ));
}
