use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

// handle_alert reacts to the alert in #pajlada.
pub async fn handle_alert(
    msg: PrivmsgMessage,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
) {
    // Only reason I do it like this is so that i can test it easier xd.
    let pajlada = "pajlada";
    let pajbot = "pajbot";
    let pajbot_id = "82008718";
    let alert_message = "pajaS 🚨 ALERT";

    // testing
    // let pajlada = "nourylul";
    // let pajbot = "nourylul";
    // let pajbot_id = "31437432";
    // let alert_message = "testxd";

    if msg.channel_login == pajlada
        && msg.sender.login == pajbot
        && msg.sender.id == pajbot_id
        && msg.message_text == alert_message
        && msg.is_action
    {
        client
            .privmsg(
                pajlada.to_owned(),
                "/me PAJAW 🚨 seems like a good alert, it has my support.".to_owned(),
            )
            .await
            .unwrap();
    };
}
