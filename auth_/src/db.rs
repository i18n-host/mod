#![allow(non_snake_case, clippy::too_many_arguments)]

use mysql_macro::{Result, q1};

pub async fn signInLog(
  uid: u64,
  ip: impl AsRef<[u8]>,
  timezone: i8,
  dpi: u8,
  w: u16,
  h: u16,
  arch: impl AsRef<str>,
  model: impl AsRef<str>,
  cpu_num: u32,
  gpu: impl AsRef<str>,
  os_v1: u32,
  os_v2: u32,
  brand: impl AsRef<str>,
  os_name: impl AsRef<str>,
  browser_name: impl AsRef<str>,
  browser_ver: u32,
  browser_lang: impl AsRef<str>,
) -> Result<u64> {
  let sql = format!(
    "SELECT signInLog({uid},?,{timezone},{dpi},{w},{h},?,?,{cpu_num},?,{os_v1},{os_v2},?,?,?,{browser_ver},?)"
  );
  Ok(q1!(
    sql,
    ip.as_ref(),
    arch.as_ref(),
    model.as_ref(),
    gpu.as_ref(),
    brand.as_ref(),
    os_name.as_ref(),
    browser_name.as_ref(),
    browser_lang.as_ref()
  ))
}

#[macro_export]
macro_rules! signInLog {
  ($uid:expr,$ip:expr,$timezone:expr,$dpi:expr,$w:expr,$h:expr,$arch:expr,$model:expr,$cpu_num:expr,$gpu:expr,$os_v1:expr,$os_v2:expr,$brand:expr,$os_name:expr,$browser_name:expr,$browser_ver:expr,$browser_lang:expr) => {
    $crate::signInLog(
      $uid,
      $ip,
      $timezone,
      $dpi,
      $w,
      $h,
      $arch,
      $model,
      $cpu_num,
      $gpu,
      $os_v1,
      $os_v2,
      $brand,
      $os_name,
      $browser_name,
      $browser_ver,
      $browser_lang,
    )
    .await?
  };
}
