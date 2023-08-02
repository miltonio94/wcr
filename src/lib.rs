use std::error::Error;

type MyResult<R> = Result<R, Box<dyn Error>>;

struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

fn get_args() -> MyResult<Config> {
    //
    Ok(Config {
        files: vec!["kasfjl".to_string()],
        lines: true,
        words: true,
        bytes: false,
        chars: true,
    })
}

fn run(config: Config) -> MyResult<()> {
    //
    Ok(())
}
