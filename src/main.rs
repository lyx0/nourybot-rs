extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::env;

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let login_name = dotenv!("TWITCH_USERNAME").to_owned();
    let oauth_token = dotenv!("TWITCH_OAUTH").to_owned();
    let config =
        ClientConfig::new_simple(StaticLoginCredentials::new(login_name, Some(oauth_token)));

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);
        }
    });

    client.join("pajlada".to_owned()).unwrap();

    client
        .say("pajlada".to_owned(), "test xd".to_owned())
        .await
        .unwrap();

    join_handle.await.unwrap();

    println!("{}", dotenv!("TWITCH_USERNAME"));
    println!("{}", dotenv!("TWITCH_OAUTH"));
}
