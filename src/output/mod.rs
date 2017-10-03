use crypto::elliptic::credentials::{YSecretKey, YPublicKey};
use crypto::encryption::symmetric::YIV;
use amount::YAmount;
use data::YData;

#[derive(Clone)]
pub struct YOutput {
  pub sender: YPublicKey,
  pub receiver: YPublicKey,
  pub amount: YAmount,
  pub data: Option<YData>,
  pub custom: Option<[u8; 32]>,
}

impl YOutput {
  pub fn new(
    sk: &YSecretKey,
    receiver: &YPublicKey,
    amount: YAmount,
    custom: Option<[u8; 32]>) -> Option<YOutput> {
    let sender = sk.public_key();
    let max_amount = YAmount::max_value();
    if amount > max_amount {
      return None;
    }
    Some(YOutput {
      sender: sender.clone(),
      receiver: receiver.clone(),
      amount: amount.clone(),
      data: None,
      custom: custom,
    })
  }

  pub fn with_data(
    sk: &YSecretKey,
    receiver: &YPublicKey,
    iv: YIV,
    plain: &[u8],
    custom: Option<[u8; 32]>) -> Option<YOutput> {
    let sender = sk.public_key();
    if let Some(data) = YData::new(sk, receiver, iv, plain) {
      Some(YOutput {
        sender: sender.clone(),
        receiver: receiver.clone(),
        amount: data.amount(),
        data: Some(data),
        custom: custom,
      })
    } else {
      None
    }
  }

  pub fn to_bytes(&self) -> Vec<u8> { unreachable!() }

  pub fn from_bytes(b: &[u8]) -> Option<YOutput> { unreachable!() }
}
