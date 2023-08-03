use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

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
        .help("Show line count")
        .short("l")
        .long("lines")
        .value_name("LINES"),
    )
    .arg(
      Arg::with_name("words")
        .help("Show word count")
        .short("w")
        .long("words")
        .value_name("WORDS"),
    )
    .arg(
      Arg::with_name("bytes")
        .help("Show byte count")
        .short("c")
        .long("bytes")
        .value_name("BYTES"),
    )
    .arg(
      Arg::with_name("chars")
        .help("Show character count")
        .short("m")
        .long("chars")
        .value_name("CHARS")
        .conflicts_with("bytes"),
    )
    .get_matches();

  let mut lines = matches.is_present("lines");
  let mut words = matches.is_present("words");
  let mut bytes = matches.is_present("bytes");
  let chars = matches.is_present("chars");

  if [lines, words, bytes, chars].iter().all(|v| v == &false) {
    lines = true;
    words = true;
    bytes = true;
  }

  Ok(Config {
    files: matches.values_of_lossy("files").unwrap(),
    lines,
    words,
    bytes,
    chars,
  })
}

pub fn run(config: Config) -> MyResult<()> {
  let num_files = config.files.len();
  println!("{:#?}", config);
  for (file_num, file) in config.files.iter().enumerate() {
    match open(&file) {
      Err(err) => eprintln!("{}: {}", file, err),
      Ok(buffer) => {
        println!("opened file {}", &file);
      },
    };
  }
  Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}
