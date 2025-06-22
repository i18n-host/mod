#![allow(non_snake_case, clippy::too_many_arguments)]

use mysql_macro::{Result, q1};

pub async fn signInLog(
  uid: u64,
  ip: impl AsRef<[u8]>,
  timezone: i8,
  dpi: u8,
  w: u16,
  h: u16,
  os_ver: impl AsRef<str>,
  arch: impl AsRef<str>,
  cpu_num: u32,
  gpu: impl AsRef<str>,
  brand: impl AsRef<str>,
  os_name: impl AsRef<str>,
  browser_name: impl AsRef<str>,
  browser_ver: u32,
  browser_lang: impl AsRef<str>,
) -> Result<u64> {
  let sql = format!(
    "SELECT signInLog({uid},?,{timezone},{dpi},{w},{h},?,?,{cpu_num},?,?,?,?,{browser_ver},?)"
  );
  Ok(q1!(
    sql,
    ip.as_ref(),
    os_ver.as_ref(),
    arch.as_ref(),
    gpu.as_ref(),
    brand.as_ref(),
    os_name.as_ref(),
    browser_name.as_ref(),
    browser_lang.as_ref()
  ))
}

#[macro_export]
macro_rules! signInLog {
  ($uid:expr,$ip:expr,$timezone:expr,$dpi:expr,$w:expr,$h:expr,$os_ver:expr,$arch:expr,$cpu_num:expr,$gpu:expr,$brand:expr,$os_name:expr,$browser_name:expr,$browser_ver:expr,$browser_lang:expr) => {
    $crate::signInLog(
      $uid,
      $ip,
      $timezone,
      $dpi,
      $w,
      $h,
      $os_ver,
      $arch,
      $cpu_num,
      $gpu,
      $brand,
      $os_name,
      $browser_name,
      $browser_ver,
      $browser_lang,
    )
    .await?
  };
}
