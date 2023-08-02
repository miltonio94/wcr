use clap::{App, Arg};
use std::error::Error;

type MyResult<R> = Result<R, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("M")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .value_name("LINES"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .value_name("WORDS"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES"),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .value_name("CHARS"),
        )
        .get_matches();
    Ok(Config {
        files: vec!["kasfjl".to_string()],
        lines: true,
        words: true,
        bytes: false,
        chars: true,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    //
    Ok(())
}
