use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::str;
//use std::time::Instant;

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
        //println!("raw_char_as_str: {}", raw_char_as_str);
        if (0x20..=0x7E).contains(&raw_char) {
            // TODO: Fix this garbage .unwrap() replace with '?'
            conv_line.push_str(raw_char_as_str.unwrap());
        } else {
            conv_line.push('.');
        }
    }
    conv_line
}

fn line_all_zero(raw_line: &[u8]) -> bool {
    let mut sum: u64 = 0;
    for i in raw_line {
        sum += *i as u64;
    }
    sum == 0
}

fn hexdump(file: String, file_len: usize, offset: usize) -> io::Result<()> {
    // Open file and read length or default bytes at a time
    let mut f = File::open(file)?;
    let mut address: u64 = 0x0;
    let mut i: usize = 0;
    let mut _remainder = 0;
    let mut rem_printed = false;
    let mut is_zero_line_printed = false;
    let mut is_skip_line_printed = false;
    let mut line: [u8; 0x10] = [0; 0x10];
    //let mut prev_line: [u8; 0x10] = [0; 0x10];

    if offset >= file_len {
        let custom_error = Error::new(ErrorKind::Other, "offset >= file_len, bailing");
        eprintln!("offset >= file_len ({} >= {}), bailing", offset, file_len);
        return Err(custom_error);
    }

    while i < file_len {
        // TODO: handle case at the end where this might overflow the array bounds...
        if i + 0x10 > file_len {
            _remainder = (i + 0x10) - file_len;
            if !rem_printed {
                println!("remainder: {}", _remainder);
                rem_printed = true;
            }
        }
        f.read_exact(&mut line).expect("Didn't read 0x10 bytes");
        let conv_line_as_str = convert_line(&line);
        let _is_line_zero = line_all_zero(&line);
        if !_is_line_zero || !is_zero_line_printed {
            // Default print no format string provided
            let line = format!(
                    "{:08x}  {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}  |{}|",
                    address,
                    line[0],
                    line[1],
                    line[2],
                    line[3],
                    line[4],
                    line[5],
                    line[6],
                    line[7],
                    line[8],
                    line[9],
                    line[10],
                    line[11],
                    line[12],
                    line[13],
                    line[14],
                    line[15],
                    conv_line_as_str
                );
            println!("{}", line);
            if _is_line_zero {
                is_zero_line_printed = true;
                is_skip_line_printed = false;
            } else {
                is_zero_line_printed = false;
            }
        } else if !is_skip_line_printed {
            println!("*");
            is_skip_line_printed = true;
        }
        address += 0x10;
        i += 0x10;
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
            // If length is not provided by the user then use the file len.
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
