use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

pub async fn handle_message(
    msg: PrivmsgMessage,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
) {
    let cmd_params: Vec<&str> = msg.message_text.split(" ").collect();
    let command = cmd_params[0];

    //
    println!("{:?}", command);

    if command == "()ping" {
        client
            .say(msg.channel_login.to_owned(), "Pong!".to_owned())
            .await
            .unwrap();
    };
    // for s in split {
    //     println!("{}", s);
    //     // let command = s.chars().nth(0);
    //     // println!("command: {:?}", command)
    // }
    // println!("{:?}", msg);
}
