use aok::{Result, OK};
use tracing::info;

#[tokio::test]
async fn test() -> Result<()> {
  xboot::init().await?;
  info!("test");
  // assert!(false == captcha_verify::run(&captcha.id, &[0, 0, 0, 0, 0, 0]).await?);
  // assert!(captcha_verify::run(&captcha.id, &click_x_y).await?);
  OK
}

// #[test]
// fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }
