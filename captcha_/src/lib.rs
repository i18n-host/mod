use aok::Result;
use click_captcha::{ICO_LI, PosLi};
use pb_jelly::Message;
use rand::{Rng, SeedableRng, rngs::StdRng};
use s_::EMPTY;
use xbin::concat;
use xkv::{
  R,
  fred::{interfaces::KeysInterface, types::Expiration},
};

const SIZE: u32 = 350;

pub const R_CAPTCHA: &[u8] = b"captcha:";

pub struct Captcha {
  pub id: Vec<u8>,
  pub img: Vec<u8>,
  pub tip: Vec<u8>,
}

pub async fn verify(id: &[u8], click_pos_li: &[u32]) -> Result<String> {
  let key = concat!(R_CAPTCHA, id);
  if let Ok(Some::<Vec<u8>>(bin)) = xerr::ok!(R.get(key).await)
    && !bin.is_empty()
    && let Ok::<PosLi, _>(pos_li) = xerr::ok!(pc::d(&bin))
    && click_captcha::verify(pos_li, click_pos_li)
  {
    R.set::<(), _, _>(key, EMPTY, Some(Expiration::EX(60)), None, false)
      .await?;

    return Ok(ub64::b64e(id));
  }

  let captcha = captcha().await?;

  ih::err::bin(
    ih::Captcha {
      id: captcha.id,
      img: captcha.img,
      tip: captcha.tip,
    }
    .serialize_to_vec(),
  )
}

pub async fn captcha() -> Result<Captcha> {
  let (img, meta) = click_captcha::webp(SIZE, SIZE, ICO_LI)?;

  let mut rng = StdRng::from_rng(&mut rand::rng());

  let id: [u8; 16] = rng.random();

  R.set::<(), _, _>(
    concat!(R_CAPTCHA, id),
    &pc::e(meta.pos_li)?[..],
    Some(Expiration::EX(60)),
    None,
    false,
  )
  .await?;

  let tip = meta
    .ico_li
    .into_iter()
    .map(|i| ICO_LI[i])
    .collect::<Vec<&'static str>>()
    .join("|")
    .as_bytes()
    .into();

  Ok(Captcha {
    id: id.into(),
    img: img.into(),
    tip,
  })
}
