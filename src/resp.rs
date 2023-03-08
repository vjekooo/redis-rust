pub enum RespValues {
  SimpleString(String),
  Error(Vec<u8>),
  Integer(i64),
  BulkString(Vec<u8>),
  Array(Vec<RespValues>),
}

impl RespValues {
  pub fn serialize(self, buffer: &mut Vec<u8>) {
    match self {
      RespValues::Array(values) => {
        buffer.push(b'*');
        buffer.append(&mut format!("{}", values.len()).into_bytes());
        buffer.push('\r' as u8);
        buffer.push('\n' as u8);
        for value in values {
          value.serialize(buffer)
        }
      }
      RespValues::BulkString(mut data) => {
        buffer.push(b'$');
        buffer.append(&mut format!("{}", data.len()).into_bytes());
        buffer.push('\r' as u8);
        buffer.push('\n' as u8);
        buffer.append(&mut data);
        buffer.push('\r' as u8);
        buffer.push('\n' as u8);
      }
      _ => unimplemented!(),
    }
  }
}
