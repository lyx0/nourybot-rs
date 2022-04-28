extern crate dotenv;
mod config;

#[macro_use]
extern crate dotenv_codegen;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

#[tokio::main]
pub async fn main() {
    let cfg = config::Config::new();
    let config =
        ClientConfig::new_simple(StaticLoginCredentials::new(cfg.username, Some(cfg.oauth)));

    // xd
    let pajlada = "pajlada";
    let pajbot = "pajbot";
    let pajbot_id = "82008718";
    let alert_message = "pajaS 🚨 ALERT";

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    client.join("pajlada".to_owned()).unwrap();
    client.join("nourylul".to_owned()).unwrap();
    client
        .say("nourylul".to_owned(), "RaccAttack TeaTime".to_owned())
        .await
        .unwrap();
    client
        .say("nourybot".to_owned(), "RaccAttack".to_owned())
        .await
        .unwrap();

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    if msg.channel_login == pajlada
                        && msg.sender.login == pajbot
                        && msg.sender.id == pajbot_id
                        && msg.message_text == alert_message
                        && msg.is_action
                    {
                        client
                            .say("pajlada".to_owned(), "🦀 🚨 ALARM IS GONE".to_owned())
                            .await
                            .unwrap();
                    };
                }
                ServerMessage::Whisper(msg) => {
                    println!("[whisper] {}: {}", msg.sender.name, msg.message_text);
                }
                _ => {}
            }
        }
    });

    join_handle.await.unwrap();

    println!("{}", dotenv!("TWITCH_USERNAME"));
    println!("{}", dotenv!("TWITCH_OAUTH"));
}
