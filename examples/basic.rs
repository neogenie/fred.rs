#![allow(clippy::disallowed_names)]
#![allow(clippy::let_underscore_future)]

use fred::prelude::*;

#[tokio::main]
async fn main() -> Result<(), RedisError> {
  // create a config from a URL
  let config = RedisConfig::from_url("redis://username:password@foo.com:6379/1")?;
  // see the `Builder` interface for more information
  let client = Builder::from_config(config).build()?;
  // callers can manage the tokio task driving the connections
  let connection_task = client.init().await?;
  // convert response types to most common rust types
  let foo: Option<String> = client.get("foo").await?;
  println!("Foo: {:?}", foo);

  client
    .set("foo", "bar", Some(Expiration::EX(1)), Some(SetOptions::NX), false)
    .await?;

  // or use turbofish. the first type is always the response type.
  println!("Foo: {:?}", client.get::<Option<String>, _>("foo").await?);

  client.quit().await?;
  // calling quit ends the connection and event listener tasks
  let _ = connection_task.await;
  Ok(())
}
