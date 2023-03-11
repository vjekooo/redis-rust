
use crate::error::Error;

pub fn parse_response(buffer: &[u8], bytes_read: usize) -> Result<&str, Error> {
  let mut response = "";

  if buffer.is_empty() {
    return Err(Error {});
  }

  if buffer[0] == (b'-') {
    return Err(Error {});
  }

  if buffer[0] == (b'+') {
    response = std::str::from_utf8(&buffer[1..3]).unwrap();
  }

  if buffer[0] == (b'$') {
    response = std::str::from_utf8(&buffer[4..bytes_read]).unwrap();
  }

  if buffer[0] == (b':') {
    response = std::str::from_utf8(&buffer[1..3]).unwrap();
  }

  println!("Response, {}", response);

  Ok(response)
}