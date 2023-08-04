use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::str;
//use std::time::Instant;

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
    /// Two-byte hexadecimal display.  Display the input offset in hexadecimal, followed by eight,
    /// space separated, four column, zero-filled, two-byte quantities of input data, in hexadecimal, per line.
    #[arg(short = 'x', long)]
    hex: Option<bool>,
    /// For each input file, hexdump sequentially copies the input to standard output,
    /// transforming the data according to the format strings specified by the -e and
    /// -f options, in the order that they were specified.
    file: String,
}

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

// Helper function to see if two buffers equal each other
fn vecs_match(b1: &[u8], b2: &[u8]) -> bool {
    for (i, item) in b1.iter().enumerate() {
        if b2[i] != *item {
            return false;
        }
    }
    true
}

// Helper function to print out each line with default formatting.
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

// Main function that reads and dumps the conents of the file
fn hexdump(file: String, file_len: usize, offset: usize) -> io::Result<()> {
    let mut f = File::open(file)?;
    let mut line: [u8; READ_LEN] = [0; READ_LEN];
    let mut _prev_line: [u8; READ_LEN] = [0; READ_LEN];

    if offset >= file_len {
        let custom_error = Error::new(ErrorKind::Other, "offset >= file_len, bailing");
        eprintln!("offset >= file_len ({} >= {}), bailing", offset, file_len);
        return Err(custom_error);
    }

    if offset & 0xF != 0x0 {
        // We need to align this address to be READ_LEN based for the below algorithm to work.
    }
    // TODO: align address to be READ_LEN based
    let mut address: usize = offset;

    let mut _is_skip_line_printed = false;
    while address < file_len {
        if file_len < READ_LEN {
            let mut var_len_line: Vec<u8> = vec![0; file_len];
            f.read_exact(&mut var_len_line).unwrap_or_else(|_| {
                panic!("{}", &format!("Didn't read {} bytes", READ_LEN).to_owned())
            });
            print_bin(&mut var_len_line, address);
            address += file_len;
        } else if address + READ_LEN > file_len {
            let mut _remainder = file_len % READ_LEN;
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
    println!("{:08x}", address);
    Ok(())
}

fn main() {
    //let now = Instant::now();
    let args = Args::parse();
    let mut length: usize = 0;
    let mut bytes_to_skip: usize = 0;

    if let Some(in_length) = args.length {
        println!("Value for length:{}", in_length);
        length = in_length;
    }
    if let Some(in_offset) = args.offset {
        println!("Value for offset:{}", in_offset);
        bytes_to_skip = in_offset;
    }
    if let Some(hex) = args.hex {
        println!("Value for hex:{hex}");
    }

    // Check that the file exists before we try and open it.
    let metadata = std::fs::metadata(args.file.clone());
    match metadata {
        Ok(metadata) => {
            if length == 0 {
                // TODO: Fix this garbage .unwrap() replace with '?'
                length = metadata.len().try_into().unwrap();
            }
        }
        Err(_) => {
            eprintln!("file does not exist, exiting.");
            return;
        }
    }
    let _ = hexdump(args.file, length, bytes_to_skip);
    //println!("Execution time was: {:#?}", now.elapsed());
}
