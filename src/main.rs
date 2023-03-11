mod resp;
mod ttl;
mod client;
mod error;
mod parse;

use crate::ttl::TimeToLive;
use crate::client::run_client;

use async_std::io;

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




