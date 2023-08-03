Hexdump written in rust.  This is mainly being done as a learning excercise.
## Hexdump

## TODO 

Add support to skip a repeated line even if it isn't all 00.
Add support for -v, which will print repeated lines.
Add support for different representations of the data (hex, octal, dec, bin).
Add support for format strings.
Add support for -n or --length to be 20 or 0x20 (32 decimal) currently only
  20 works.

Add tests to test the functionality you have working.  This will help as I add
new features to be able to have confidence the new code doesn't break anything.
