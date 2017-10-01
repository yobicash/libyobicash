use curve25519_dalek::edwards::ValidityCheck;
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use amount::YAmount;
use data::YData;

pub struct YOutput {
  pub sender: YPoint,
  pub receiver: YPoint,
  pub amount: YAmount,
  pub data: Option<YData>,
  pub custom: Option<[u8; 32]>,
}

impl YOutput {
  pub fn new(
    g: &YPoint,
    sk: &YScalar,
    receiver: &YPoint,
    amount: &YAmount,
    custom: Option<[u8; 32]>) -> Option<YOutput> {
    if !g.is_valid() || receiver.is_valid() {
      return None;
    }
    let sender = g*sk;
    let max_amount = YAmount::max_value();
    if *amount > max_amount {
      return None;
    }
    Some(YOutput {
      sender: sender,
      receiver: *receiver,
      amount: amount.clone(),
      data: None,
      custom: custom,
    })
  }

  pub fn with_data(
    g: &YPoint,
    sk: &YScalar,
    receiver: &YPoint,
    iv: &[u8],
    plain: &[u8],
    custom: Option<[u8; 32]>) -> Option<YOutput> {
    if !g.is_valid() || receiver.is_valid() {
      return None;
    }
    let sender = g*sk;
    if let Some(data) = YData::new(g, sk, receiver, iv, plain) {
      Some(YOutput {
        sender: sender,
        receiver: *receiver,
        amount: data.amount(),
        data: Some(data),
        custom: custom,
      })
    } else {
      None
    }
  }
}
