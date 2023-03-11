
use crate::resp::RespValues;

pub enum TimeToLive {
  Ex(String),
  Px(String),
  Exat(String),
  Pxat(String),
}

impl TimeToLive {
  pub fn init(self) -> Vec<RespValues> {
    match self {
      TimeToLive::Ex(value) => {
        vec![
          RespValues::BulkString(b"EX".to_vec()),
          RespValues::BulkString(value.into_bytes()),
        ]
      }
      TimeToLive::Px(value) => {
        vec![
          RespValues::BulkString(b"PX".to_vec()),
          RespValues::BulkString(value.into_bytes()),
        ]
      }
      TimeToLive::Exat(value) => {
        vec![
          RespValues::BulkString(b"Exat".to_vec()),
          RespValues::BulkString(value.into_bytes()),
        ]
      }
      TimeToLive::Pxat(value) => {
        vec![
          RespValues::BulkString(b"Pxat".to_vec()),
          RespValues::BulkString(value.into_bytes()),
        ]
      }
    }
  }
}
