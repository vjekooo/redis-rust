use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::io;
use futures::{AsyncReadExt, AsyncWriteExt};

use crate::resp::RespValues;
use crate::error::Error;
use crate::ttl::TimeToLive;
use crate::parse::parse_response;

pub struct SetValues {
  pub key: String,
  pub value: String,
  pub ttl: Option<TimeToLive>,
  pub get: Option<String>,
}

pub struct Client {
  stream: TcpStream,
}

impl Client {
  pub async fn new<A: ToSocketAddrs>(address: A) -> Result<Client, io::Error> {
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
  pub async fn set(
    &mut self,
    SetValues { key, value, ttl, get }: SetValues
  ) -> Result<String, Error> {
    let key_clone = key.clone();
    let existing_value;

    if get.is_some() {
      let mut client = run_client(None).await?;
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
  pub async fn delete(&mut self, key: String) -> Result<String, Error> {
    let keys = split_string(&key)?;
    let mut values = vec![RespValues::BulkString(b"DEL".to_vec())];
    for value in keys {
      values.push(RespValues::BulkString(value.to_string().into_bytes()));
    }
    let command = RespValues::Array(values);
    write_to_db(&mut self.stream, command).await
  }
  pub async fn rename(&mut self, key: String, new_key: String) -> Result<String, Error> {
    let command = RespValues::Array(vec![
      RespValues::BulkString(b"RENAME".to_vec()),
      RespValues::BulkString(key.into_bytes()),
      RespValues::BulkString(new_key.into_bytes()),
    ]);
    write_to_db(&mut self.stream, command).await
  }
}

pub async fn run_client(port: Option<u16>) -> io::Result<Client> {
  let port = port.unwrap_or(6379);
  let str_value = format!("{}{}", "localhost:", port);
  let client = Client::new(str_value).await?;
  Ok(client)
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