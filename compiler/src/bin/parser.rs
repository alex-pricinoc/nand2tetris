use std::{env, process};

use compiler::compiler::{Compiler, Mode, Result, Source};

fn main() -> Result {
    let file = env::args().nth(1).unwrap_or_else(|| {
        println!("help: parser <input jack file>");
        process::exit(1);
    });

    let compiler = Compiler::new(Source::File(file), Mode::Xml);
    compiler.compile()?;

    Ok(())
}
