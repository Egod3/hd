Hexdump written in rust.  This is mainly being done as a learning excercise.
## Hexdump

## TODO
Add support for format strings.
Optimize the solution so it is closer to hd (GNU C implementation) in performance.

## Done

Add support for different representations of the data (hex, octal, decimal, or ascii).
    (-C hex, -o two-bytes octal, -b one byte octal, -d decimal, -C ascii, -c character)
Add support for -n or --length to be 20 or 0x20 (32 decimal) currently only 20 works.
Add support for -s offset to start reading file at provided offset.
Add support for -v, which will print repeated lines.
Add tests to test the functionality you have working.  This will help as I add
new features to be able to have confidence the new code doesn't break anything.
Add support for skipping duplicate lines.
Add support for printing unaligned lengths.
Add support for length.
Add support to dump bins.

## Basic tests:

# command 1 & command 2 & test command
diff <(./target/release/hd -C ./target/release/hd) <(hd ./target/release/hd) --ignore-trailing-space

diff <(hexdump ./target/release/hd) <(./target/release/hd ./target/release/hd); result=$?; if [[ $result = 0 ]];then  echo "Test Passed"; else echo "Test Failed";  fi;

diff <( hexdump test/test0.bin -c) <(./target/release/hd test/test0.bin -c); result=$?; if [[ $result = 0 ]];then  echo "Test Passed"; else echo "Test Failed";  fi;

diff <( hexdump test/test0.bin -d) <(./target/release/hd test/test0.bin -d); result=$?; if [[ $result = 0 ]];then  echo "Test Passed"; else echo "Test Failed";  fi;

# command 1 & command 2 & test command
diff <( hd test/test0.bin) <(./target/release/hd -C test/test0.bin) --ignore-trailing-space
diff <(hexdump test/test1.bin) <(./target/release/hd test/test1.bin) --ignore-trailing-space
diff <(hexdump test/test1.bin -o) <(./target/release/hd test/test1.bin -o) --ignore-trailing-space

# Cargo tests
cargo test -- --show-output
