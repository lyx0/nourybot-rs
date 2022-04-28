use dotenv::dotenv;

pub struct Config {
    pub username: String,
    pub oauth: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        let username = dotenv!("TWITCH_USERNAME").to_owned();
        let oauth = dotenv!("TWITCH_OAUTH").to_owned();
        return Config {
            username: username,
            oauth: oauth,
        };
    }
}

// pub fn new_config() -> Config {
//     dotenv().ok();
//     let username = dotenv!("TWITCH_USERNAME").to_owned();
//     let oauth = dotenv!("TWITCH_OAUTH").to_owned();
//     return Config {
//         username: username,
//         oauth: oauth,
//     };
// }
