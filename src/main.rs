extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    println!("{}", dotenv!("TWITCH_USERNAME"));
    println!("{}", dotenv!("TWITCH_OAUTH"));
}
