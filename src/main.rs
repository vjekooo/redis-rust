mod resp;

use crate::resp::RespValues;

use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use futures::{AsyncReadExt, AsyncWriteExt};

#[async_std::main]
async fn main() -> io::Result<()> {
  let mut client = run_client().await?;
  client
    .set(
      "vjeko".into(),
      "keks".into(),
      Some(TimeToLive::Px("6000".into())),
      Some("GET".into()),
    )
    .await;

  Ok(())
}

fn parse_response(buffer: &[u8], bytes_read: usize) -> Result<&str, Error> {
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

#[derive(Debug)]
struct Error {}

impl std::convert::From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error {}
  }
}

struct Client {
  stream: TcpStream,
}

impl Client {
  async fn new<A: ToSocketAddrs>(address: A) -> Result<Client, io::Error> {
    let stream = TcpStream::connect(address).await?;
    Ok(Client { stream })
  }
}

impl Client {
  async fn get(&mut self, key: String) -> Result<String, Error> {
    let command = RespValues::Array(vec![
      RespValues::BulkString(b"GET".to_vec()),
      RespValues::BulkString(key.into_bytes()),
    ]);
    write_to_db(&mut self.stream, command).await
  }
  async fn set(
    &mut self,
    key: String,
    value: String,
    ttl: Option<TimeToLive>,
    get: Option<String>,
  ) -> Result<String, Error> {
    let key_clone = key.clone();
    let existing_value;

    if get.is_some() {
      let mut client = run_client().await?;
      let response = client.get(key).await?;
      existing_value = response;
    }

    let mut values = vec![
      RespValues::BulkString(b"SET".to_vec()),
      RespValues::BulkString(key_clone.into_bytes()),
      RespValues::BulkString(value.into_bytes()),
    ];

    if ttl.is_some() {
      let ttl_values = ttl.unwrap().init();
      for value in ttl_values {
        values.push(value);
      }
    }
    let command = RespValues::Array(values);
    let response = write_to_db(&mut self.stream, command).await?;
    Ok(response)
  }
  async fn delete(&mut self, key: String) -> Result<String, Error> {
    let keys = split_string(&key)?;
    let mut values = vec![RespValues::BulkString(b"DEL".to_vec())];
    for value in keys {
      values.push(RespValues::BulkString(value.to_string().into_bytes()));
    }
    let command = RespValues::Array(values);
    write_to_db(&mut self.stream, command).await
  }
  async fn rename(&mut self, key: String, new_key: String) -> Result<String, Error> {
    let command = RespValues::Array(vec![
      RespValues::BulkString(b"RENAME".to_vec()),
      RespValues::BulkString(key.into_bytes()),
      RespValues::BulkString(new_key.into_bytes()),
    ]);
    write_to_db(&mut self.stream, command).await
  }
}

async fn write_to_db(stream: &mut TcpStream, command: RespValues) -> Result<String, Error> {
  let mut buffer = vec![];
  command.serialize(&mut buffer);
  stream.write_all(&buffer).await?;
  let bytes_read = stream.read(&mut buffer).await?;
  let response = parse_response(&buffer, bytes_read)?;
  Ok(response.to_owned())
}

fn split_string(key: &String) -> Result<Vec<&str>, Error> {
  let v: Vec<&str> = key.split(' ').collect();
  Ok(v)
}

async fn run_client() -> io::Result<Client> {
  let client = Client::new("localhost:6379").await?;
  Ok(client)
}

enum TimeToLive {
  Ex(String),
  Px(String),
  Exat(String),
  Pxat(String),
}

impl TimeToLive {
  fn init(self) -> Vec<RespValues> {
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

