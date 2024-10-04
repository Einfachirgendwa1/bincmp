use core::panic;
use std::{
    fs::File,
    io::{Read, Write},
};

use clap::{ArgAction, Parser};

#[derive(Parser)]
struct Cli {
    file: String,

    #[arg(long, short)]
    output: Option<String>,

    #[arg(long, short, action = ArgAction::SetTrue)]
    verbose: bool,
}

fn main() {
    let Cli {
        file,
        output,
        verbose,
    } = Cli::parse();

    let output = output.unwrap_or("out".to_string());

    let mut file = File::open(file).unwrap();
    let mut content = Vec::new();

    file.read_to_end(&mut content).unwrap();

    let mut out = Vec::new();

    for mut line in content.split(|c| *c as char == '\n') {
        if line.contains(&('#' as u8)) {
            line = line.split(|c| *c as char == '#').next().unwrap();
        }

        let mut i = 0;
        while i + 1 < line.len() {
            let f = |c| {
                if c == ' ' {
                    return c as u8;
                }
                if '0' <= c && c <= '9' {
                    c as u8 - '0' as u8
                } else if 'A' <= c && c <= 'F' {
                    c as u8 - 'A' as u8 + 10
                } else {
                    panic!("Invalid char '{c}' ({})", c as u8);
                }
            };
            let a = f(line[i] as char);
            if a == ' ' as u8 {
                i += 1;
                continue;
            }
            let b = f(line[i + 1] as char);
            out.push(a * 16 + b);
            i += 2;
        }
    }

    if verbose {
        println!("{out:?}");
    }

    let mut outfile = File::create(output).unwrap();
    outfile.write_all(out.as_slice()).unwrap();
}
