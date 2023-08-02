use clap::Parser;
use std::str;

// TODO: add a trace32 option that auto-formats the same way d.dump in Trace32 looks.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Interpret only length bytes of input.
    #[arg(short = 'n', long)]
    length: Option<u64>,
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
        if raw_char >= 0x20 && raw_char <= 0x7E {
            conv_line.push_str(raw_char_as_str.unwrap());
        } else {
            conv_line.push('.');
        }
    }
    conv_line
}

fn main() {
    println!("Welcome to hexdump (hd)");

    let args = Args::parse();

    if let Some(length) = args.length {
        println!("Value for length:{}", length);
    }
    if let Some(hex) = args.hex {
        println!("Value for hex:{hex}");
    }

    // Open file and read length or default bytes at a time
    // TODO: Fix this garbage .unwrap() replace with '?'
    let _bytes = std::fs::read(args.file).unwrap();
    let mut address: u64 = 0x0;
    let mut i = 0;
    // TODO: handle case at the end where this might overflow the array bounds...
    while i < _bytes.len() {
        // Default print no format string provided
        // TODO: Fix this garbage .unwrap() replace with '?'
        let conv_line_as_str = convert_line(&_bytes[i..i + 16]);
        let line = format!(
            "{:08x}  {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}  | {}",
            address,
            _bytes[i],
            _bytes[i + 1],
            _bytes[i + 2],
            _bytes[i + 3],
            _bytes[i + 4],
            _bytes[i + 5],
            _bytes[i + 6],
            _bytes[i + 7],
            _bytes[i + 8],
            _bytes[i + 9],
            _bytes[i + 10],
            _bytes[i + 11],
            _bytes[i + 12],
            _bytes[i + 13],
            _bytes[i + 14],
            _bytes[i + 15],
            conv_line_as_str
        );
        println!("{}", line);
        address += 0x10;
        i += 0x10;
    }
}
