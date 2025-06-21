use aok::{OK, Void};
use http::HeaderMap;

#[iat::captcha]
pub async fn captcha(headers: &HeaderMap) -> Void {
  dbg!(headers);
  OK
}

pub async fn manual_captcha(headers: &HeaderMap) -> Void {
  if let Some(c) = headers.get("c") {
    dbg!(c);
    return OK;
  }
  ih::captcha()
}
