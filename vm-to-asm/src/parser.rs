use crate::code;
use std::error::Error;
use std::io::Write;

pub fn run(input: &str, out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    for line in instructions(input) {
        let instruction = line.parse()?;
        let instructions = code::write(instruction);

        for (i, ins) in instructions.into_iter().enumerate() {
            if i > 0 {
                writeln!(out)?;
            }

            write!(out, "{}", ins)?;
        }
        writeln!(out, " // {}", line)?;
    }

    writeln!(out, "{}", code::INFINITE_LOOP)?;

    Ok(())
}

fn instructions(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(|l| {
            let offset = l.find("//").unwrap_or(l.len());
            let line = l[0..offset].trim();
            line
        })
        .filter(|&l| !l.is_empty())
}
