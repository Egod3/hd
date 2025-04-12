use clap::Parser;
use clap_num::maybe_hex;
use std::fs::File;
use std::io;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{Error, ErrorKind, Read};
use std::str;
use std::time::Instant;

#[cfg(test)]
mod tests;

const READ_LEN: usize = 0x10;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Interpret only length bytes of input.
    #[arg(short = 'n', long, value_parser=maybe_hex::<usize>)]
    length: Option<usize>,
    /// Skip offset bytes from the beginning of the input.
    #[arg(short = 's', long, value_parser=maybe_hex::<usize>)]
    offset: Option<usize>,
    /// File to hexdump.
    file: String,
    /// Cause hexdump to display all input data. Without the -v option, any
    /// number of groups of output lines, that are identical, are replaced with
    /// a line comprised of a single asterisk.
    #[arg(short = 'v')]
    print_all_lines: bool,
    /// Canonical hex+ASCII display.
    #[arg(short = 'C', long)]
    canonical: bool,
    /// two-byte hexadecimal display
    #[arg(short = 'x', long)]
    two_bytes_hex: bool,
}

// convert a u8 array into a String, for the right side of the dump
fn convert_to_string(raw_line: &[u8]) -> String {
    let mut conv_line: String = Default::default();
    for i in 0..raw_line.len() {
        let raw_char_as_str = str::from_utf8(&raw_line[i..i + 1]);
        let raw_char = raw_line[i];
        if (0x20..=0x7E).contains(&raw_char) {
            // TODO: Fix this .unwrap() replace with '?'
            conv_line.push_str(raw_char_as_str.unwrap());
        } else {
            conv_line.push('.');
        }
    }
    conv_line
}

// Function to see if two buffers equal each other
fn vecs_match(b1: &[u8], b2: &[u8]) -> bool {
    for (i, item) in b1.iter().enumerate() {
        if b2[i] != *item {
            return false;
        }
    }
    true
}

// Function to print out each line with default formatting.
fn print_bin(line: &mut [u8], address: usize, canonical: bool, two_bytes_hex: bool) {
    let line_len = line.len();
    if line_len > READ_LEN {
        eprintln!("line_len > READ_LEN, quitting print_bin()");
        return;
    }

    let mut new_line;
    // -C canonical format with |ascii| printed to the right of the hex
    if canonical {
        new_line = format!("{:08x} ", address);
        if line_len > 0 {
            for (i, _) in (0..READ_LEN).enumerate() {
                if i < line_len {
                    let spaces = if i == 8 { "  " } else { " " };
                    new_line = format! {"{}{}{:02x}", new_line, spaces, line[i]};
                } else {
                    let spaces = if i == 8 { "    " } else { "   " };
                    new_line = format! {"{}{}", new_line, spaces};
                }
            }
            new_line = format!("{}  |{}|", new_line, convert_to_string(line));
            println!("{}", new_line);
        } else {
            println!("{}", new_line);
        }
    // -x two bytes hexadecimal
    } else if two_bytes_hex && line_len > 0 {
        new_line = format!("{:07x}", address);
        let mut i = 0;
        while i < READ_LEN {
            if i < line_len && i + 1 < line_len {
                let spaces = "    ";
                new_line = format! {"{}{}{:02x}{:02x}", new_line, spaces, line[i + 1], line[i]};
            } else {
                let spaces = if i == 8 { "   " } else { "  " };
                new_line = format! {"{}{}", new_line, spaces};
            }
            i += 2;
        }
        println!("{}", new_line);
    // Default print no format string provided
    } else if line_len > 0 {
        new_line = format!("{:07x}", address);
        let mut i = 0;
        while i < READ_LEN {
            if i < line_len && i + 1 < line_len {
                let spaces = " ";
                new_line = format! {"{}{}{:02x}{:02x}", new_line, spaces, line[i + 1], line[i]};
            } else {
                let spaces = if i == 8 { "   " } else { "  " };
                new_line = format! {"{}{}", new_line, spaces};
            }
            i += 2;
        }
        println!("{}", new_line);
    } else {
        new_line = format!("{:07x}", address);
        println!("{}", new_line);
    }
}

// Open the file and print its content to stdout
fn hexdump(
    file: String,
    req_bytes_to_dump: usize,
    offset: usize,
    print_all_lines: bool,
    canonical: bool,
    two_bytes_hex: bool,
) -> io::Result<usize> {
    let mut f = File::open(&file)?;
    let mut line: [u8; READ_LEN] = [0; READ_LEN];
    let mut _prev_line: [u8; READ_LEN] = [0; READ_LEN];
    let mut _file_length: usize = std::fs::metadata(file)?.len().try_into().unwrap();
    let mut bytes_left_to_dump: usize = req_bytes_to_dump;
    let mut address: usize = offset;

    if offset > _file_length {
        let custom_error = Error::new(ErrorKind::Other, "offset > _file_length, exiting");
        eprintln!(
            "offset >= _file_length ({} > {}), exiting",
            offset, _file_length
        );
        return Err(custom_error);
    }

    if bytes_left_to_dump == 0 {
        return Ok(0);
    }

    if bytes_left_to_dump + offset > _file_length {
        bytes_left_to_dump = _file_length - offset;
    }

    if address != 0x0 {
        f.seek(SeekFrom::Start(address.try_into().unwrap()))?;
    }

    let mut _is_skip_line_printed = false;
    loop {
        // Read a subline where bytes_left_to_dump is less than READ_LEN
        if bytes_left_to_dump < READ_LEN {
            let mut var_len_line: Vec<u8> = vec![0; bytes_left_to_dump];
            f.read_exact(&mut var_len_line).unwrap_or_else(|_| {
                panic!(
                    "Didn't read {} bytes read {} bytes",
                    bytes_left_to_dump,
                    var_len_line.len()
                )
            });
            print_bin(&mut var_len_line, address, canonical, two_bytes_hex);
            address += bytes_left_to_dump;
            break;
        // Normal case, read a full READ_LEN line
        } else {
            f.read_exact(&mut line).unwrap_or_else(|_| {
                panic!(
                    "{}",
                    &format!("Didn't read {} bytes on line {:?}", READ_LEN, line).to_owned()
                )
            });
            if print_all_lines {
                print_bin(&mut line, address, canonical, two_bytes_hex);
            } else {
                let _is_line_same = vecs_match(&line, &_prev_line);
                if _is_line_same && !_is_skip_line_printed {
                    println!("*");
                    _is_skip_line_printed = true;
                // This line matched the previous line so skip printing.
                } else if _is_line_same && _is_skip_line_printed {
                } else {
                    print_bin(&mut line, address, canonical, two_bytes_hex);
                    _is_skip_line_printed = false;
                }
                for (i, item) in line.iter().enumerate() {
                    _prev_line[i] = *item;
                }
            }
            address += READ_LEN;
            bytes_left_to_dump -= READ_LEN;
        }
    }
    // Return the bytes dumped wrapped in a Result
    Ok(address - offset)
}

fn main() {
    let now = Instant::now();
    let args = Args::parse();
    let mut length: usize = 0;
    let mut _file_length: usize = 0;
    let mut bytes_to_skip: usize = 0;

    if let Some(in_length) = args.length {
        println!("Value for length:{}", in_length);
        length = in_length;
    }
    if let Some(in_offset) = args.offset {
        println!("Value for offset:{}", in_offset);
        bytes_to_skip = in_offset;
    }

    // Check that the file exists before we try and open it.
    let metadata = std::fs::metadata(&args.file);
    let _result;
    match metadata {
        Ok(metadata) => {
            // TODO: Fix this .unwrap() replace with '?'
            _file_length = metadata.len().try_into().unwrap();
            if length == 0 {
                length = _file_length
            } else if length > _file_length {
                length = _file_length;
            }
            _result = hexdump(
                args.file,
                length,
                bytes_to_skip,
                args.print_all_lines,
                args.canonical,
                args.two_bytes_hex,
            );
        }
        Err(_) => {
            eprintln!("file does not exist, exiting.");
        }
    }
    if args.print_all_lines {
        println!("Execution time: {:#?}", now.elapsed());
    }
}
