use async_std::io;

#[derive(Debug)]
pub struct Error {}

impl std::convert::From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error {}
  }
}