mod resp;
mod ttl;
mod client;
mod error;
mod parse;

use std::env;
use crate::ttl::TimeToLive;
use crate::client::run_client;

use async_std::io;

#[async_std::main]
async fn main() -> io::Result<()> {

  let args: Vec<String> = env::args().collect();

  println!("Args print {:?}", args);

  let mut client = run_client().await?;
  client
    .set(
      "vjeko".into(),
      "keks".into(),
      Some(TimeToLive::Px("6000".into())),
      Some("GET".into()),
    )
    .await.expect("Its a message");

  Ok(())
}