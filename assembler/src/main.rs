use assembler::Config;
use std::{env, fs::File, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Assembling file: {}", config.file_path);

    let file_name = config.file_path.as_str().split_once('.').unwrap().0;

    let mut file = File::create(format!("{file_name}.hack")).unwrap();

    if let Err(e) = assembler::run(config, &mut file) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
