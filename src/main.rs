use async_std::io;
use async_std::net::TcpStream;
use futures::{AsyncReadExt, AsyncWriteExt};

#[async_std::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("localhost:6379").await?;
    let mut buffer = vec![0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    parse_response(&buffer[0..bytes_read]);
    println!("{:?}", parse_response(&buffer[0..bytes_read]));
    Ok(())
}

fn parse_response(buffer: &[u8]) -> Result<&str, String> {
    if buffer.is_empty() {
        return Err("Empty buffer".into());
    }

    if buffer[0] == (b'-') {
        return Err(format!(
            "Error response: {:?}",
            &buffer[1..buffer.len() - 2]
        ));
    }

    Ok(std::str::from_utf8(&buffer[1..buffer.len() - 2]).unwrap())
}

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
    async fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        let command = ResponseValues::Array(vec![
            ResponseValues::BulkString(b"SET".to_vec()),
            ResponseValues::BulkString(b"Vjeko".to_vec()),
            ResponseValues::BulkString(b"Test".to_vec()),
        ]);
        let mut buffer = vec![];
        command.serialize(&mut buffer);
        self.stream.write_all(&buffer).await?;
        Ok(())
    }
}

enum ResponseValues {
    SimpleString(String),
    Error(Vec<u8>),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<ResponseValues>),
}

impl ResponseValues {
    fn serialize(self, buffer: &mut Vec<u8>) {
        match self {
            ResponseValues::Array(values) => {
                buffer.push(b'*');
                buffer.append(&mut format!("{}", values.len()).into_bytes());
                buffer.push('\r' as u8);
                buffer.push('\n' as u8);
                for value in values {
                    value.serialize(buffer)
                }
            }
            ResponseValues::BulkString(mut data) => {
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
