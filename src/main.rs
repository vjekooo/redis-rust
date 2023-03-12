mod resp;
mod ttl;
mod client;
mod error;
mod parse;

use crate::ttl::TimeToLive;
use crate::client::{run_client, SetValues};

use async_std::io;

#[async_std::main]
async fn main() -> io::Result<()> {

  let mut client = run_client(None).await?;

  let set = SetValues {
    key: String::from("Works"),
    value: String::from("Yes"),
    ttl: Some(TimeToLive::Px(String::from("6000"))),
    get: Some(String::from("Works")),
  };

  client
    .set(set)
    .await.expect("Client action failed");

  Ok(())
}