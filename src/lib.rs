use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::{Add, AddAssign};

type MyResult<R> = Result<R, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  lines: bool,
  words: bool,
  bytes: bool,
  chars: bool,
}

#[derive(Debug, Copy, Clone)]
struct FileInfo {
  line_num: usize,
  word_count: usize,
  char_count: usize,
  byte_size: usize,
}

impl Add for FileInfo {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self {
      line_num: self.line_num + other.line_num,
      word_count: self.word_count + other.word_count,
      char_count: self.char_count + other.char_count,
      byte_size: self.byte_size + other.byte_size,
    }
  }
}

impl AddAssign for FileInfo {
  fn add_assign(&mut self, other: Self) {
    self.line_num += other.line_num;
    self.word_count += other.word_count;
    self.char_count += other.char_count;
    self.byte_size += other.byte_size;
  }
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
        .long("lines"),
    )
    .arg(
      Arg::with_name("words")
        .help("Show word count")
        .short("w")
        .long("words"),
    )
    .arg(
      Arg::with_name("bytes")
        .help("Show byte count")
        .short("c")
        .long("bytes"),
    )
    .arg(
      Arg::with_name("chars")
        .help("Show character count")
        .short("m")
        .long("chars")
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

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
  //
  let mut file_content = String::new();
  file.read_to_string(&mut file_content)?;

  Ok(FileInfo {
    line_num: file_content.lines().count(),
    word_count: file_content.split_whitespace().count(),
    char_count: file_content.chars().count(),
    byte_size: file_content.len(),
  })
}

pub fn run(config: Config) -> MyResult<()> {
  let mut total_file_info = FileInfo {
    line_num: 0,
    word_count: 0,
    byte_size: 0,
    char_count: 0,
  };
  for file in config.files.iter() {
    match open(&file) {
      Err(err) => eprintln!("{}: {}", file, err),
      Ok(buffer) => {
        let file_info = count(buffer)?;
        total_file_info = file_info + total_file_info;
        println!(
          "{}{}{}{}{}",
          format_field(file_info.line_num, config.lines),
          format_field(file_info.word_count, config.words),
          format_field(file_info.byte_size, config.bytes),
          format_field(file_info.char_count, config.chars),
          if file == "-" {
            "".to_string()
          } else {
            format!(" {}", file)
          }
        )
      },
    };
  }
  if config.files.len() > 1 {
    println!(
      "{}{}{}{} total",
      format_field(total_file_info.line_num, config.lines),
      format_field(total_file_info.word_count, config.words),
      format_field(total_file_info.byte_size, config.bytes),
      format_field(total_file_info.char_count, config.chars),
    );
  }
  Ok(())
}

fn format_field(value: usize, show: bool) -> String {
  if show {
    format!("{:>8}", value)
  } else {
    "".to_string()
  }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}
