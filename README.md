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

# Cargo tests
cargo test -- --show-output
