#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_convert_line_one() {
        use crate::convert_to_string;
        let line = "This is a line of ascii";
        let bin = &line.as_bytes();
        let ascii = convert_to_string(bin);
        println!("ascii: {}", ascii);
        assert_eq!(ascii, line);
    }

    #[test]
    fn test_convert_line_two() {
        use crate::convert_to_string;
        let answer = "9.....=..D..A..F";
        let buf = vec![
            0x39, 0xa8, 0x03, 0x00, 0x00, 0x85, 0x3d, 0x0d, 0xe5, 0x44, 0x00, 0xdc, 0x41, 0x08,
            0x81, 0x46,
        ];
        let ascii = convert_to_string(&buf);
        println!("ascii: {}", ascii);
        assert_eq!(ascii, answer);
    }

    #[test]
    fn test_vecs_match() {
        use crate::vecs_match;
        let answer = false;
        let b1 = "This is a string".as_bytes();
        let b2 = "This is a strinG".as_bytes();
        let ascii = vecs_match(b1, b2);
        assert_eq!(ascii, answer);
    }

    #[test]
    fn test_vecs_match_two() {
        use crate::vecs_match;
        let answer = true;
        let b1 = vec![
            0x93, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x11, 0x0, 0xab, 0x0, 0x0, 0x0,
        ];
        let b2 = vec![
            0x93, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x11, 0x0, 0xab, 0x0, 0x0, 0x0,
        ];
        let ascii = vecs_match(&b1, &b2);
        assert_eq!(ascii, answer);
    }

    #[test]
    fn test_bad_file_path() {
        use crate::HdOptions;
        use crate::hexdump;
        #[allow(unused_imports)]
        use crate::print_bin;
        let f1 = "test/test0_does_not_exist.bin".to_string();
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: true,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        assert_eq!(hexdump(f1.clone(), 10, 10, &opt).is_err(), true);
    }

    #[test]
    fn test_bad_file_path_one() {
        use crate::HdOptions;
        use crate::hexdump;
        #[allow(unused_imports)]
        use crate::print_bin;
        let f1 = "test0.bin".to_string();
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: true,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        assert_eq!(hexdump(f1.clone(), 10, 10, &opt).is_err(), true);
    }

    #[test]
    fn test_hd_offset_gt_bytes_to_dump_one() {
        use crate::HdOptions;
        use crate::hexdump;
        #[allow(unused_imports)]
        use crate::print_bin;
        let f1 = "test/test0.bin".to_string();
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: true,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };

        let status = hexdump(f1.clone(), 10, 11, &opt);
        println!("status: {:?}", status);
        assert_eq!(status.is_ok(), true);
    }
    #[test]
    fn test_hd_offset_gt_bytes_to_dump_two() {
        use crate::HdOptions;
        use crate::hexdump;
        let f1 = "test/test0.bin".to_string();
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: true,
            two_bytes_dec: true,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        assert_eq!(hexdump(f1.clone(), 976, 0, &opt).is_ok(), true,);
    }
    #[test]
    fn test_hd_offset_gt_bytes_to_dump_three() {
        use crate::hexdump;
        let f1 = "test/test0.bin".to_string();
        use crate::HdOptions;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        assert_eq!(hexdump(f1.clone(), 20, 2000, &opt).is_err(), true,);
    }
    #[test]
    fn test_hd_offset_gt_bytes_to_dump_four() {
        use crate::hexdump;
        let f1 = "test/test0.bin".to_string();
        use crate::HdOptions;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: true,
        };
        assert_eq!(hexdump(f1.clone(), 0, 0xa1, &opt).is_ok(), true,);
    }

    #[test]
    fn test_hd_bytes_to_dump_ten() {
        use crate::HdOptions;
        use crate::hexdump;
        let f1 = "test/test0.bin".to_string();
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let bytes_to_dump = 10;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            bytes_to_dump, bytes_to_dump
        );
        let result = hexdump(f1.clone(), bytes_to_dump, 0, &opt.clone());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }
    #[test]
    fn test_hd_bytes_to_dump_twenty() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 20;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            bytes_to_dump, bytes_to_dump
        );
        let result = hexdump(f1.clone(), bytes_to_dump, 0, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_one() {
        use crate::hexdump;
        let f1 = "test/test0.bin".to_string();
        use crate::HdOptions;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: true,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let bytes_to_dump = 1024;
        let offset = 0x0; // 0
        let mut _file_length: usize = std::fs::metadata(&f1)
            .expect("not a file")
            .len()
            .try_into()
            .expect("not the length");
        let min_bytes = std::cmp::min(offset + bytes_to_dump, _file_length - offset);
        println!(
            "hd {} bytes, file length {} make sure {} lines are skipped and {} bytes are dumped",
            bytes_to_dump, _file_length, offset, min_bytes
        );
        let result = hexdump(f1.clone(), bytes_to_dump, 0, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), min_bytes);
    }
    #[test]
    fn test_hd_bytes_to_dump_nine_ninty() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_octal: false,
            two_bytes_dec: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 990;
        const MAX_BYTES_TO_DUMP: usize = 976;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            MAX_BYTES_TO_DUMP, MAX_BYTES_TO_DUMP
        );
        // Special case where we try and read past the end of the file
        let result = hexdump(f1.clone(), bytes_to_dump, 0, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), MAX_BYTES_TO_DUMP);
    }

    #[test]
    fn test_hd_bytes_to_dump_eight_hunderd_big_offset() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let bytes_to_dump = 0x320; // 800
        let f2 = "test/test1.bin".to_string();
        let mut _file_length: usize = std::fs::metadata(&f2)
            .expect("not a file")
            .len()
            .try_into()
            .expect("not the length");
        let offset = 0x2e0; // 736
        let min_bytes = std::cmp::min(offset + bytes_to_dump, _file_length - offset);
        println!(
            "hd {} bytes, file length {} make sure {} lines are skipped and {} bytes are dumped",
            bytes_to_dump, _file_length, offset, min_bytes
        );
        // Special case where we try and read past the end of the file
        let result = hexdump(f2.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        // Only read until the end of file.
        assert_eq!(result.unwrap(), min_bytes);
    }

    #[test]
    fn test_hd_no_squeezing() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: true,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test1.bin".to_string();
        let bytes_to_dump = 10;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            bytes_to_dump, bytes_to_dump
        );
        let result = hexdump(f1.clone(), bytes_to_dump, 0, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_no_squeezing_two() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: true,
            two_bytes_dec: false,
            two_bytes_octal: true,
            two_bytes_hex: false,
        };
        let f1 = "test/test1.bin".to_string();
        let bytes_to_dump = 32;
        let offset = 0x2e0; // 736
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            bytes_to_dump, bytes_to_dump
        );
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_no_squeezing_three() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: true,
            two_bytes_octal: false,
            two_bytes_dec: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test1.bin".to_string();
        let mut _file_length: usize = std::fs::metadata(&f1)
            .expect("not a file")
            .len()
            .try_into()
            .expect("not the length");
        let bytes_to_dump = 0x332; // 818
        let offset = 0x2e7; // 743

        // Dump from offset -> bytes_to_dump or _file_length - offset which ever is smaller
        let min_bytes = std::cmp::min(offset + bytes_to_dump, _file_length - offset);
        println!(
            "hd {} bytes, file length {} make sure {} lines are skipped and {} bytes are dumped",
            bytes_to_dump, _file_length, offset, min_bytes
        );
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), min_bytes);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_one() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: true,
            no_squeezing: true,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 25;
        let offset = 0;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_two() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 10;
        let offset = 5;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_three() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 500;
        let offset = 0;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_four() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 18;
        let offset = 1;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_five() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: true,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 240;
        let offset = 111;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_six() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: true,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 93;
        let offset = 9;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_seven() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 9;
        let offset = 16;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_eight() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: true,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: false,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 40;
        let offset = 16;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_nine() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: true,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: true,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 32;
        let offset = 14;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset_ten() {
        use crate::HdOptions;
        use crate::hexdump;
        let opt: HdOptions = HdOptions {
            canonical: false,
            one_byte_char: false,
            one_byte_octal: false,
            no_squeezing: false,
            two_bytes_dec: false,
            two_bytes_octal: false,
            two_bytes_hex: true,
        };
        let f1 = "test/test0.bin".to_string();
        let bytes_to_dump = 93;
        let offset = 9;
        println!("hd {} bytes starting at {}", bytes_to_dump, offset);
        let result = hexdump(f1.clone(), bytes_to_dump, offset, &opt);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), bytes_to_dump);
    }

    #[test]
    fn test_hd_against_hexdump_from_gnu() {
        println!("Run this command:");
        println!(
            "  diff <(./target/release/hd ./target/release/hd) <(hexdump ./target/release/hd); result=$?; if [[ $result = 0 ]];then  echo \"Test Passed\"; else echo \"Test Failed\";  fi;"
        );
        println!(
            "  diff <(./target/release/hd -C ./target/release/hd) <(hd ./target/release/hd) --ignore-trailing-space; result=$?; if [[ $result = 0 ]];then  echo \"Test Passed\"; else echo \"Test Failed\";  fi;"
        );
        println!(
            "  diff <(./target/release/hd -c ./target/release/hd) <(hd ./target/release/hd -c) --ignore-trailing-space; result=$?; if [[ $result = 0 ]];then  echo \"Test Passed\"; else echo \"Test Failed\";  fi;"
        );
        println!(
            "  diff <(./target/release/hd -b ./target/release/hd) <(hd ./target/release/hd -b) --ignore-trailing-space; result=$?; if [[ $result = 0 ]];then  echo \"Test Passed\"; else echo \"Test Failed\";  fi;"
        );
        println!(
            "  diff <(./target/release/hd -d ./target/release/hd) <(hd ./target/release/hd -d) --ignore-trailing-space; result=$?; if [[ $result = 0 ]];then  echo \"Test Passed\"; else echo \"Test Failed\";  fi;"
        );
        println!(
            "  diff <(./target/release/hd -o ./target/release/hd) <(hd ./target/release/hd -o) --ignore-trailing-space; result=$?; if [[ $result = 0 ]];then  echo \"Test Passed\"; else echo \"Test Failed\";  fi;"
        );
        println!(
            "  diff <(./target/release/hd -x ./target/release/hd) <(hd ./target/release/hd -x) --ignore-trailing-space; result=$?; if [[ $result = 0 ]];then  echo \"Test Passed\"; else echo \"Test Failed\";  fi;"
        );
    }
}
