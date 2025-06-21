use std::net::IpAddr;

use aok::{OK, Result};
use xkv::{
  R,
  fred::interfaces::{KeysInterface, SortedSetsInterface},
};
use set_cookie::{MAX, SET_COOKIE};
use http::HeaderMap;
use ctx_::SetHeader;
use cookie_b::Browser;
use xbin::concat;

use crate::{
  BrowserMeta, UA,
  r::{R_BROWSER_META, R_BROWSER_USER, R_USER_BROWSER},
};

#[iat::captcha]
pub async fn mail(
  address: &str,
  password: &str,
  headers: &HeaderMap,
  set_header: &SetHeader,
  browser: &Browser,
) -> Result<()> {
  /*
    根据域名id，邮箱确认用户，不同网站可以复用一套后台
  */

  let host = header_host::host(headers)?;
  let cookie = set_cookie::new(host);
  let uid = 12;

  let ua = UA.parse(
    headers
      .get("user-agent")
      .and_then(|v| v.to_str().ok())
      .unwrap_or_default(),
  );

  let ip = headers
    .get("x-forwarded-for")
    .and_then(|v| v.to_str().ok())
    .and_then(|value: &str| value.split(',').next().map(str::trim))
    .and_then(|v| v.parse::<IpAddr>().ok());

  let client_version = ua.client.version.unwrap_or_default();
  let os_version = ua.os.version.unwrap_or_default();

  let browser_meta = BrowserMeta {
    ip,
    brand: ua.client.family,
    ver: client_version,
    os: ua.os.family,
    os_ver: os_version,
  };

  let now = sts::sec() as f64;
  let uid_bin = &intbin::to_bin(uid)[..];

  /*

  browser -> user 浏览器登录了的用户
  user -> browser 用户在哪些设备上登录

  */

  let browser_user = concat!(R_BROWSER_USER, browser.bin);
  let p = R.pipeline();
  let _: () = p
    .zadd(browser_user, None, None, false, false, (now, uid_bin))
    .await?;
  let _: () = p.expire(browser_user, MAX as _, None).await?;
  let _: () = p
    .zadd(
      concat!(R_USER_BROWSER, uid_bin),
      None,
      None,
      false,
      false,
      (now, &browser.bin[..]),
    )
    .await?;

  let _: () = p
    .set(
      concat!(R_BROWSER_META, browser.bin),
      pc::e(browser_meta)?,
      None,
      None,
      false,
    )
    .await?;

  let _: () = p.last().await?;
  set_header.push(SET_COOKIE, cookie.set_max_for_js("u", ub64::u64_b64(uid)));
  OK
}
