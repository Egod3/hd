Hexdump written in rust.  This is mainly being done as a learning excercise.
## Hexdump

## TODO 

Add support for -v, which will print repeated lines.
Add support for different representations of the data (hex, octal, dec, bin).
Add support for format strings.
Add support for -n or --length to be 20 or 0x20 (32 decimal) currently only
  20 works.

Add tests to test the functionality you have working.  This will help as I add
new features to be able to have confidence the new code doesn't break anything.

## Basic tests you can run:

# command 1 & command 2 & test command
cargo run -- /mnt/NAS/data/git/tools/hd/target/release/hd > hd-rs.release.txt
hd /mnt/NAS/data/git/tools/hd/target/release/hd > hd.release.txt
diff hd.release.txt hd-rs.release.txt

# command 1 & command 2 & test command
cargo run -- test0.bin > hd-rs.test0.txt
hd test0.bin > hd.test0.txt 
diff hd.test0.txt hd-rs.test0.txt

cargo test -- --show-output
