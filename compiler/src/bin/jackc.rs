use std::{env, path::PathBuf};

use compiler::compiler::{Compiler, Mode, Result, Source};

fn main() -> Result {
    let file = env::args().nth(1).unwrap_or_else(|| String::from("."));

    let source = if PathBuf::from(&file).is_file() {
        Source::File(file)
    } else {
        Source::Directory(file)
    };

    let compiler = Compiler::new(source, Mode::Vm);
    compiler.compile()?;

    Ok(())
}
