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

pub async fn test(
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

  let uid = 12344;

  let ip: Vec<u8> = x_read_ip::get(headers);

  let (os_v1, os_v2) = if os_v1 == 0 && os_v2 == 0 {
    let os_ver = ua.os.version.as_deref().unwrap_or_default();
    let mut iter = os_ver.split('.');
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

  let gpu: String = gpu.replace(", Unspecified Version", "");

  let brand: String = ua.device.brand.as_deref().unwrap_or_default().to_string();

  let os_name: String = headers
    .get("sec-ch-ua-platform")
    .and_then(|v| v.to_str().ok())
    .map(|s| s.replace('"', ""))
    .unwrap_or_else(|| ua.os.family.to_string());

  let browser_name: String = ua.client.family.to_string();

  let browser_ver: u32 = ua
    .client
    .version
    .as_deref()
    .unwrap_or_default()
    .split('.')
    .next()
    .unwrap_or_default()
    .parse::<u32>()
    .unwrap_or_default();

  let browser_lang: &str = headers
    .get("accept-language")
    .and_then(|v| v.to_str().ok())
    .unwrap_or_default()
    .split(',')
    .next()
    .unwrap_or_default();

  signInLog(
    uid,
    ip,
    timezone,
    dpi,
    w,
    h,
    arch,
    model,
    cpu_num,
    &gpu,
    os_v1,
    os_v2,
    &brand,
    &os_name,
    &browser_name,
    browser_ver,
    browser_lang,
  )
  .await
}
