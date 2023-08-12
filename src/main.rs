use clap::Parser;
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

// TODO: add a trace32 option that auto-formats the same way d.dump in Trace32 looks.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Interpret only length bytes of input.
    #[arg(short = 'n', long)]
    length: Option<usize>,
    /// Skip offset bytes from the beginning of the input.
    #[arg(short = 's', long)]
    offset: Option<usize>,
    /// File to hexdump
    file: String,
    /// Cause hexdump to display all input data. Without the -v option, any
    /// number of groups of output lines, that are identical, are replaced with
    /// a line comprised of a single asterisk.
    #[arg(short = 'v')]
    print_all_lines: bool,
    // Two-byte hexadecimal display.  Display the input offset in hexadecimal, followed by eight,
    // space separated, four column, zero-filled, two-byte quantities of input data, in hexadecimal, per line.
    //#[arg(short = 'x', long)]
    //hex: Option<bool>,
}

// convert a u8 array into a String, for the right side of the dump
fn convert_line(raw_line: &[u8]) -> String {
    let mut conv_line: String = Default::default();
    for i in 0..raw_line.len() {
        let raw_char_as_str = str::from_utf8(&raw_line[i..i + 1]);
        let raw_char = raw_line[i];
        if (0x20..=0x7E).contains(&raw_char) {
            // TODO: Fix this garbage .unwrap() replace with '?'
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
fn print_bin(line: &mut [u8], address: usize) {
    let line_len = line.len();
    if line_len > READ_LEN {
        eprintln!("line_len > READ_LEN, quitting print_bin()");
        return;
    }

    // Default print no format string provided
    let mut new_line = format!("{:08x} ", address);
    for (_, i) in (0..READ_LEN).enumerate() {
        if i < line_len {
            let spaces = if i == 8 { "  " } else { " " };
            new_line = format! {"{}{}{:02x}", new_line, spaces, line[i]};
        } else {
            let spaces = if i == 8 { "    " } else { "   " };
            new_line = format! {"{}{}", new_line, spaces};
        }
    }
    new_line = format!("{}  |{}|", new_line, convert_line(line));
    println!("{}", new_line);
}

// Function that opens the file and dumps its content
fn hexdump(
    file: String,
    req_bytes_to_dump: usize,
    offset: usize,
    print_all_lines: bool,
) -> io::Result<usize> {
    let mut f = File::open(&file)?;
    let mut line: [u8; READ_LEN] = [0; READ_LEN];
    let mut _prev_line: [u8; READ_LEN] = [0; READ_LEN];
    let mut _file_length: usize = std::fs::metadata(file)?.len().try_into().unwrap();
    let mut bytes_to_dump: usize = req_bytes_to_dump;

    if offset >= req_bytes_to_dump {
        let custom_error = Error::new(ErrorKind::Other, "offset >= req_bytes_to_dump, bailing");
        eprintln!(
            "offset >= req_bytes_to_dump ({} >= {}), bailing",
            offset, req_bytes_to_dump
        );
        return Err(custom_error);
    }

    if req_bytes_to_dump > _file_length {
        println!("req_bytes_to_dump > _file_length so we are truncating the dump amount");
        bytes_to_dump = _file_length;
    }

    let mut address: usize = offset;

    if address != 0x0 {
        f.seek(SeekFrom::Start(address.try_into().unwrap()))?;
    }

    let mut _is_skip_line_printed = false;
    while address < bytes_to_dump {
        if bytes_to_dump < READ_LEN {
            let mut var_len_line: Vec<u8> = vec![0; bytes_to_dump];
            f.read_exact(&mut var_len_line).unwrap_or_else(|_| {
                panic!("{}", &format!("Didn't read {} bytes", READ_LEN).to_owned())
            });
            print_bin(&mut var_len_line, address);
            address += bytes_to_dump;
        } else if address + READ_LEN > bytes_to_dump {
            let mut _remainder = bytes_to_dump % READ_LEN;
            let mut var_len_line: Vec<u8> = vec![0; _remainder];
            f.read_exact(&mut var_len_line).unwrap_or_else(|_| {
                panic!("{}", &format!("Didn't read {} bytes", READ_LEN).to_owned())
            });
            print_bin(&mut var_len_line, address);
            address += _remainder;
        } else {
            f.read_exact(&mut line).unwrap_or_else(|_| {
                panic!("{}", &format!("Didn't read {} bytes", READ_LEN).to_owned())
            });
            if print_all_lines {
                address += READ_LEN;
                print_bin(&mut line, address);
            } else {
                let _is_line_same = vecs_match(&line, &_prev_line);
                if _is_line_same && !_is_skip_line_printed {
                    println!("*");
                    _is_skip_line_printed = true;
                // This line matched the previous line so skip printing.
                } else if _is_line_same && _is_skip_line_printed {
                } else {
                    print_bin(&mut line, address);
                    _is_skip_line_printed = false;
                }
                address += READ_LEN;
                for (i, item) in line.iter().enumerate() {
                    _prev_line[i] = *item;
                }
            }
        }
    }
    println!("{:08x}", address);
    // Return the bytes dumped wrapped in a Result
    Ok(address - offset)
}

fn main() {
    let now = Instant::now();
    let args = Args::parse();
    let mut length: usize = 0;
    let mut _file_length: usize = 0;
    let mut bytes_to_skip: usize = 0;
    let mut _print_all_lines: bool = true;

    if let Some(in_length) = args.length {
        println!("Value for length:{}", in_length);
        length = in_length;
    }
    if let Some(in_offset) = args.offset {
        println!("Value for offset:{}", in_offset);
        bytes_to_skip = in_offset;
    }
    _print_all_lines = args.print_all_lines;
    //if let Some(hex) = args.hex {
    //    println!("Value for hex:{hex}");
    //}

    // Check that the file exists before we try and open it.
    let metadata = std::fs::metadata(&args.file);
    let _result;
    match metadata {
        Ok(metadata) => {
            // TODO: Fix this garbage .unwrap() replace with '?'
            _file_length = metadata.len().try_into().unwrap();
            if length == 0 {
                length = _file_length
            } else if length > _file_length {
                length = _file_length;
            }
            _result = hexdump(args.file, length, bytes_to_skip, _print_all_lines);
        }
        Err(_) => {
            eprintln!("file does not exist, exiting.");
        }
    }
    println!("Execution time: {:#?}", now.elapsed());
}
