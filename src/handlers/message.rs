use twitch_irc::message::PrivmsgMessage;

pub fn handle_message(msg: PrivmsgMessage) {
    println!("{:?}", msg)
}
