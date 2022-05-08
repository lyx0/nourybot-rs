use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

pub async fn handle_message(
    msg: PrivmsgMessage,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
) {
    // msg_params is a Vec<&str> of words from a
    // twitch_irc::message::PrivmsgMessage
    let msg_params: Vec<&str> = msg.message_text.split(" ").collect();

    // command contains the "trigger word" for a command
    // including the prefix: e.g: ()ping
    let command: &str = msg_params[0];

    // command_stripped contains the "trigger word" for a command
    // without the prefix: e.g: ping
    let command_stripped = command.strip_prefix("()").unwrap();
    // let v: Vec<&str> = command.split(2, '').collect();

    // println!("[command] {:?}", command);
    // println!("[command_stripped] {:?}", command_stripped);

    // Log the messages
    println!(
        "PRIVMSG #{} {} :{}",
        msg.channel_login, msg.sender.login, msg.message_text
    );
    match command_stripped {
        "ping" => {
            client
                .say(msg.channel_login.to_owned(), "Pong!".to_owned())
                .await
                .unwrap();
        }
        "xd" => {
            client
                .say(msg.channel_login.to_owned(), "xd".to_owned())
                .await
                .unwrap();
        }
        _ => {}
    }
    //  if command_stripped == Some("ping") {
    //      client
    //          .say(msg.channel_login.to_owned(), "Pong!".to_owned())
    //          .await
    //          .unwrap();
    //  };
    // for s in split {
    //     println!("{}", s);
    //     // let command = s.chars().nth(0);
    //     // println!("command: {:?}", command)
    // }
    // println!("{:?}", msg);
}
