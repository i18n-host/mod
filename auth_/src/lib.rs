mod r;

pub mod err;
pub mod signin;
pub mod signup;

use simple_useragent::UserAgentParser;
use serde::{Deserialize, Serialize};

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
