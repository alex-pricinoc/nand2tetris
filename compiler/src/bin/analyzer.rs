// use compiler::Analyzer;
use compiler::CompilationEngine;
use fs::File;
use io::{BufWriter, Result};
use path::Path;
use path::PathBuf;
use std::{env, fs, io, path, process};

fn main() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("help: analyzer <input jack file..>");
        process::exit(1);
    }

    for file in args {
        let input = fs::read_to_string(&file)?;

        let output = {
            let mut name = Path::new(&file).file_stem().unwrap().to_owned();
            name.push("T");

            PathBuf::new().with_file_name(name).with_extension("xml")
        };

        let mut output = BufWriter::new(File::create(output)?);

        // let analyzer = Analyzer::new(input);

        // analyzer.generate_xml(output)?;

        let mut compilation_engine = CompilationEngine::new(&input, &mut output);
        compilation_engine.compile()?;
    }

    Ok(())
}
