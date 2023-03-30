extern crate minigrep;

use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("引数解析時に問題がありました => {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("何かしたのエラーが発生しました: \n{}", e);
        process::exit(1);
    };
}
