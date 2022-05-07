mod config;
mod handlers;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

#[tokio::main]
pub async fn main() {
    let cfg = config::load::Config::new();
    let config =
        ClientConfig::new_simple(StaticLoginCredentials::new(cfg.username, Some(cfg.oauth)));

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    client.join("pajlada".to_owned()).unwrap();
    client.join("nourylul".to_owned()).unwrap();
    client.join("nourybot".to_owned()).unwrap();
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
                    handlers::alert::handle_alert(msg.clone(), client.clone()).await;
                    handlers::message::handle_message(msg.clone(), client.clone()).await;
                }
                ServerMessage::Whisper(msg) => {
                    println!("[whisper] {}: {}", msg.sender.name, msg.message_text);
                }
                _ => {}
            }
        }
    });

    join_handle.await.unwrap();
}
