pub(crate) mod db;
pub mod err;
mod r;
pub mod signin;
pub mod signup;

use std::os;

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
  arch: &str,
  model: &str,
  cpu_num: u32,
  gpu: &str,
  os_v1: u32,
  os_v2: u32,
  headers: &http::HeaderMap,
) {
  use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
  let ua = UA.parse(
    headers
      .get("user-agent")
      .and_then(|v| v.to_str().ok())
      .unwrap_or_default(),
  );

  let browser_lang = headers
    .get("accept-language")
    .and_then(|v| v.to_str().ok())
    .unwrap_or_default()
    .split(",")
    .next()
    .unwrap_or_default();

  let ip = x_read_ip::get(headers);

  let (os_v1, os_v2) = if os_v1 == 0 && os_v2 == 0 {
    let os_ver = ua.os.version.unwrap_or_default();
    let mut iter = os_ver.split(".");
    (
      iter
        .next()
        .unwrap_or_default()
        .parse::<u32>()
        .unwrap_or_default(),
      iter
        .next()
        .unwrap_or_default()
        .parse::<u32>()
        .unwrap_or_default(),
    )
  } else {
    (os_v1, os_v2)
  };
  dbg!(model, ip, os_v1, os_v2);
  dbg!((
    timezone,
    dpi,
    w,
    h,
    arch,
    cpu_num,
    gpu.replace(", Unspecified Version", ""),
    headers
      .get("sec-ch-ua-platform")
      .and_then(|v| v.to_str().map(|i| i.replace('"', "")).ok())
      .unwrap_or(ua.os.family),
    ua.client.family,
    ua.client
      .version
      .unwrap_or_default()
      .split(".")
      .next()
      .unwrap_or_default()
      .parse::<u32>()
      .unwrap_or_default(),
    browser_lang,
  ));
}
