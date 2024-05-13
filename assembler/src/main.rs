use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::{env, fs, io, process};

fn main() -> io::Result<()> {
    let files = env::args().skip(1).collect::<Vec<_>>();

    if files.is_empty() {
        eprintln!("Usage: assembler <filename>...");
        process::exit(1);
    }

    for file in files {
        let file = Path::new(&file);
        let dest = file.with_extension("hack");

        println!(
            "assembling {} to {}",
            file.to_string_lossy(),
            dest.to_string_lossy()
        );

        let input = fs::read_to_string(file)?;

        let mut dest = BufWriter::new(File::create(dest)?);

        if let Err(e) = assembler::run(&input, &mut dest) {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    }

    Ok(())
}
