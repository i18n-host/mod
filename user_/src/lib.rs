use aok::Result;
use ctx_::User;

pub async fn name(user: &User) -> Result<String> {
  dbg!(user);
  Ok("123".into())
}
