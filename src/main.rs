use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use futures::{AsyncReadExt, AsyncWriteExt};

#[async_std::main]
async fn main() -> io::Result<()> {
    let mut client = Client::new("localhost:6379").await?;
    client.set("vjeko".into(), "keks".into()).await.unwrap();
    println!("Print, {}", client.get("vjeko".into()).await.unwrap());
    Ok(())
}

fn parse_response(buffer: &[u8], bytes_read: usize) -> Result<&str, Error> {
    let first_bytes = 4;
    if buffer.is_empty() {
        return Err(Error {});
    }

    if buffer[0] == (b'-') {
        return Err(Error {});
    }

    println!(
        "Parse, {:?}",
        std::str::from_utf8(&buffer[first_bytes..bytes_read]).unwrap()
    );

    Ok(std::str::from_utf8(&buffer[first_bytes..bytes_read]).unwrap())
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
        let mut buffer = vec![];
        command.serialize(&mut buffer);
        self.stream.write_all(&buffer).await?;
        let bytes_read = self.stream.read(&mut buffer).await?;
        let response = parse_response(&buffer, bytes_read)?;
        Ok(response.to_owned())
    }
    async fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        let command = RespValues::Array(vec![
            RespValues::BulkString(b"SET".to_vec()),
            RespValues::BulkString(key.into_bytes()),
            RespValues::BulkString(value.into_bytes()),
        ]);
        let mut buffer = vec![];
        command.serialize(&mut buffer);
        self.stream.write_all(&buffer).await?;
        let bytes_read = self.stream.read(&mut buffer).await?;
        parse_response(&buffer, bytes_read);
        Ok(())
    }
}

enum RespValues {
    SimpleString(String),
    Error(Vec<u8>),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespValues>),
}

impl RespValues {
    fn serialize(self, buffer: &mut Vec<u8>) {
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
