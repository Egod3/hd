//! hd is a hexdump clone written in Rust
use clap::Parser;
use clap_num::maybe_hex;
use std::fs::File;
use std::io;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;
use std::str;
use std::time::Instant;

#[cfg(test)]
mod tests;

/// READ_LEN constant used to check bounds and setup read buffers
const READ_LEN: usize = 0x10;

#[derive(Clone)]
struct HdOptions {
    canonical: bool,
    one_byte_char: bool,
    one_byte_octal: bool,
    no_squeezing: bool,
    two_bytes_dec: bool,
    two_bytes_octal: bool,
    two_bytes_hex: bool,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// one-byte octal display
    #[arg(short = 'b', long)]
    one_byte_octal: bool,
    /// one-byte character display
    #[arg(short = 'c', long)]
    one_byte_char: bool,
    /// canonical hex+ASCII display
    #[arg(short = 'C', long)]
    canonical: bool,
    /// two-bytes decimal display
    #[arg(short = 'd', long)]
    two_bytes_dec: bool,
    /// two-bytes octal display
    #[arg(short = 'o', long)]
    two_bytes_octal: bool,
    /// two-byte hexadecimal display
    #[arg(short = 'x', long)]
    two_bytes_hex: bool,
    /// interpret only length bytes of input
    #[arg(short = 'n', long, value_parser=maybe_hex::<usize>)]
    length: Option<usize>,
    /// skip offset bytes from the beginning of the input
    #[arg(short = 's', long, value_parser=maybe_hex::<usize>)]
    skip: Option<usize>,
    /// file to hexdump
    file: String,
    /// output identical lines
    #[arg(short = 'v', long)]
    no_squeezing: bool,
}

/// Convert a u8 array into a String, for the right side of the dump w/ -C
///
/// Args:
///   raw_line - The raw line to convert to a string
/// Return:
///   conv_line - The line as an ascii formatted string
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

/// Check to see if two buffers equal each other
///
/// Args:
///   b1 - buffer one
///   b2 - buffer two
/// Return:
///   true if the buffers match, false otherwise
fn vecs_match(b1: &[u8], b2: &[u8]) -> bool {
    for (i, item) in b1.iter().enumerate() {
        if b2[i] != *item {
            return false;
        }
    }
    true
}

/// Handle formatting for the "one-byte char" or -c format option
///
/// Args:
///   new_line: &String - The start of the format line
///   line: &[u8] - The line to be hexdumped.
///   i: usize - The index into line
/// Return:
///   fmt_line: String - The fmt_line to display
fn format_one_byte_char(new_line: &String, line: &[u8], i: usize) -> String {
    let mut fmt_line = String::new();
    if line[i] == 0 {
        let spaces = "  ";
        fmt_line = format! {"{}{}\\0", new_line, spaces};
    } else if line[i] == 7 {
        let spaces = "  ";
        fmt_line = format! {"{}{}\\a", new_line, spaces};
    } else if line[i] == 8 {
        let spaces = "  ";
        fmt_line = format! {"{}{}\\b", new_line, spaces};
    } else if line[i] == 9 {
        let spaces = "  ";
        fmt_line = format! {"{}{}\\t", new_line, spaces};
    } else if line[i] == 10 {
        let spaces = "  ";
        fmt_line = format! {"{}{}\\n", new_line, spaces};
    } else if line[i] == 11 {
        let spaces = "  ";
        fmt_line = format! {"{}{}\\v", new_line, spaces};
    } else if line[i] == 12 {
        let spaces = "  ";
        fmt_line = format! {"{}{}\\f", new_line, spaces};
    } else if line[i] == 13 {
        let spaces = "  ";
        fmt_line = format! {"{}{}\\r", new_line, spaces};
    } else if line[i] == 123 {
        let spaces = "   ";
        fmt_line = format! {"{}{}{{", new_line, spaces};
    } else if line[i] == 124 {
        let spaces = "   ";
        fmt_line = format! {"{}{}|", new_line, spaces};
    } else if line[i] == 125 {
        let spaces = "   ";
        fmt_line = format! {"{}{}}}", new_line, spaces};
    } else if line[i] == 126 {
        let spaces = "   ";
        fmt_line = format! {"{}{}~", new_line, spaces};
    } else if line[i] == 1
        || line[i] == 2
        || line[i] == 3
        || line[i] == 4
        || line[i] == 5
        || line[i] == 6
        || line[i] == 14
        || line[i] == 15
        || line[i] == 16
        || line[i] == 17
        || line[i] == 18
        || line[i] == 19
        || line[i] == 20
        || line[i] == 21
        || line[i] == 22
        || line[i] == 23
        || line[i] == 24
        || line[i] == 25
        || line[i] == 26
        || line[i] == 27
        || line[i] == 28
        || line[i] == 29
        || line[i] == 30
        || line[i] == 31
        || line[i] > 126
    {
        let spaces = " ";
        fmt_line = format! {"{}{}{:03o}", new_line, spaces, line[i]};
    } else if line[i] < 123 {
        let spaces = "   ";
        let char = char::from(line[i]);
        fmt_line = format! {"{}{}{}", new_line, spaces, char};
    }
    fmt_line
}

/// Function to print out each line
///
/// Args:
///   line - The line to hexdump
///   address - The address to print on the left side
///   option - The options passed in
/// Return:
///   true if the line is printed, false if line_len > READ_LEN
fn print_bin(line: &mut [u8], address: usize, options: &HdOptions) -> bool {
    let line_len = line.len();
    if line_len > READ_LEN {
        eprintln!("line_len > READ_LEN, quitting print_bin()");
        return false;
    }

    let mut new_line;
    // -C canonical format with |ascii| printed to the right of the hex
    if options.canonical {
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
    } else if options.two_bytes_hex && line_len > 0 {
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
    // -b one byte octal
    } else if options.one_byte_octal && line_len > 0 {
        new_line = format!("{:07x}", address);
        let mut i = 0;
        while i < READ_LEN {
            if i < line_len {
                let spaces = " ";
                new_line = format! {"{}{}{:03o}", new_line, spaces, line[i]};
            } else {
                let spaces = if i == 8 { "   " } else { "  " };
                new_line = format! {"{}{}", new_line, spaces};
            }
            i += 1;
        }
        println!("{}", new_line);
    // -o two bytes octal
    } else if options.two_bytes_octal && line_len > 0 {
        new_line = format!("{:07x}", address);
        let mut i = 0;
        while i < READ_LEN {
            if i < line_len && i + 1 < line_len {
                let spaces = "  ";
                let val: u16 = (line[i + 1] as u16) << 8 & 0xFF00 | line[i] as u16;
                new_line = format! {"{}{}{:06o}", new_line, spaces, val};
            } else {
                let spaces = if i == 8 { "   " } else { "  " };
                new_line = format! {"{}{}", new_line, spaces};
            }
            i += 2;
        }
        println!("{}", new_line);
    // -d two bytes decimal
    } else if options.two_bytes_dec && line_len > 0 {
        new_line = format!("{:07x}", address);
        let mut i = 0;
        while i < READ_LEN {
            if i < line_len && i + 1 < line_len {
                let spaces = "   ";
                let val: u16 = (line[i + 1] as u16) << 8 & 0xFF00 | line[i] as u16;
                new_line = format! {"{}{}{:05}", new_line, spaces, val};
            } else {
                let spaces = if i == 8 { "   " } else { "  " };
                new_line = format! {"{}{}", new_line, spaces};
            }
            i += 2;
        }
        println!("{}", new_line);
        // -c one byte char
    } else if options.one_byte_char && line_len > 0 {
        new_line = format!("{:07x}", address);
        let mut i = 0;
        while i < READ_LEN {
            if i < line_len {
                new_line = format_one_byte_char(&new_line, line, i);
            }
            i += 1;
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
    true
}

/// Open the file and print its content to stdout
///
/// Args:
///   file - The file to hexdump
///   req_bytes_to_dump - number of bytes to dump
///   offset - The offset to start dumping at
///   option - The format/nubmer options passed in
/// Return:
///   Result(bytes_dumped) if succes, or Err(error string)
fn hexdump(
    file: String,
    req_bytes_to_dump: usize,
    offset: usize,
    options: &HdOptions,
) -> io::Result<usize> {
    let path = Path::new(&file);
    if !path.exists() {
        let custom_error = Error::new(ErrorKind::Other, "Path does not exist, exiting");
        eprintln!("Path does not exist, exiting",);
        return Err(custom_error);
    }
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
            let status = print_bin(&mut var_len_line, address, options);
            if !status {
                let custom_error = Error::new(ErrorKind::Other, "print_bin failed, exiting");
                eprintln!("print_bin failed, exiting",);
                return Err(custom_error);
            }
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
            if options.no_squeezing {
                let status = print_bin(&mut line, address, options);
                if !status {
                    let custom_error = Error::new(ErrorKind::Other, "print_bin failed, exiting");
                    eprintln!("print_bin failed, exiting",);
                    return Err(custom_error);
                }
            } else {
                let _is_line_same = vecs_match(&line, &_prev_line);
                if _is_line_same && !_is_skip_line_printed {
                    println!("*");
                    _is_skip_line_printed = true;
                // This line matched the previous line so skip printing.
                } else if _is_line_same && _is_skip_line_printed {
                } else {
                    let status = print_bin(&mut line, address, options);
                    if !status {
                        let custom_error =
                            Error::new(ErrorKind::Other, "print_bin failed, exiting");
                        eprintln!("print_bin failed, exiting",);
                        return Err(custom_error);
                    }
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
    if let Some(in_offset) = args.skip {
        println!("Value for skip offset:{}", in_offset);
        bytes_to_skip = in_offset;
    }
    let opt: HdOptions = HdOptions {
        canonical: args.canonical,
        one_byte_char: args.one_byte_char,
        one_byte_octal: args.one_byte_octal,
        no_squeezing: args.no_squeezing,
        two_bytes_dec: args.two_bytes_dec,
        two_bytes_octal: args.two_bytes_octal,
        two_bytes_hex: args.two_bytes_hex,
    };

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
            _result = hexdump(args.file, length, bytes_to_skip, &opt);
        }
        Err(_) => {
            eprintln!("file does not exist, exiting.");
        }
    }
    if args.no_squeezing {
        println!("Execution time: {:#?}", now.elapsed());
    }
}
