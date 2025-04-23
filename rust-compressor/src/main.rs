use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,

    #[structopt(parse(from_os_str))]
    output_file: PathBuf,
}

#[derive(Debug)]
struct Token {
    offset: usize,
    length: usize,
    next_char: char,
}

// Basic LZ77 compression logic
fn compress(input: &str, window_size: usize) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect();
    let mut result: Vec<Token> = Vec::new();
    let mut pos = 0;

    while pos < chars.len() {
        let mut match_offset = 0;
        let mut match_length = 0;

        let window_start = if pos >= window_size {
            pos - window_size
        } else {
            0
        };

        for i in window_start..pos {
            let mut length = 0;

            while pos + length < chars.len()
                && chars[i + length] == chars[pos + length]
                && i + length < pos
            {
                length += 1;
            }

            if length > match_length {
                match_offset = pos - i;
                match_length = length;
            }
        }

        let next_char = if pos + match_length < chars.len() {
            chars[pos + match_length]
        } else {
            '\0'
        };

        result.push(Token {
            offset: match_offset,
            length: match_length,
            next_char,
        });

        pos += match_length + 1;
    }

    result
}

// Write tokens to a compressed file
fn write_compressed_file(tokens: &[Token], output_path: &str) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(output_path)?);

    for token in tokens {
        // Write as plain text lines: offset,length,next_char
        writeln!(
            writer,
            "{},{},{}",
            token.offset, token.length, token.next_char
        )?;
    }

    Ok(())
}

// e tokens from a file
fn decompress_from_file(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = String::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() != 3 {
            continue;
        }

        let offset: usize = parts[0].parse().unwrap_or(0);
        let length: usize = parts[1].parse().unwrap_or(0);
        let next_char = parts[2].chars().next().unwrap_or('\0');

        if offset == 0 && length == 0 {
            output.push(next_char);
        } else {
            let start = output.len() - offset;
            let substring: String = output.chars().skip(start).take(length).collect();
            output.push_str(&substring);
            if next_char != '\0' {
                output.push(next_char);
            }
        }
    }

    Ok(output)
}

fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    let args = Opt::from_args();

    // if args. != 3 {
    //     eprintln!("Usage: {} <input.txt> <output.lz77>", args[0]);
    //     std::process::exit(1);
    // }

    let input_path = &args.input_file;
    let output_path = &args.output_file;

    let mut input_file = File::open(input_path)?;
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content)?;

    let compressed = compress(&input_content, 20); // Window size can be tuned
                                                   // write_compressed_file(&compressed, output_path)?;

    println!("Compressed data written to '{:?}'", compressed);

    // Optional: Decompress to check correctness
    // let decompressed = decompress_from_file(output_path)?;
    // println!("\nDecompressed content:\n{}", decompressed);

    Ok(())
}
