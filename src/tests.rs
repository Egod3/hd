#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_convert_line() {
        use crate::convert_line;
        let line = "This is a line of ascii";
        let bin = &line.as_bytes();
        let ascii = convert_line(bin);
        println!("ascii: {}", ascii);
        assert_eq!(ascii, line);

        let answer = "9.....=..D..A..F";
        let buf = vec![
            0x39, 0xa8, 0x03, 0x00, 0x00, 0x85, 0x3d, 0x0d, 0xe5, 0x44, 0x00, 0xdc, 0x41, 0x08,
            0x81, 0x46,
        ];
        let ascii = convert_line(&buf);
        println!("ascii: {}", ascii);
        assert_eq!(ascii, answer);
    }

    #[test]
    fn test_vecs_match() {
        use crate::vecs_match;
        let mut answer = false;
        let b1 = "This is a string".as_bytes();
        let b2 = "This is a strinG".as_bytes();
        let ascii = vecs_match(b1, b2);
        assert_eq!(ascii, answer);

        answer = true;
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
    fn test_hd_bytes_to_dump() {
        use crate::hexdump;
        let f1 = "test0.bin".to_string();
        let mut len = 10;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            len, len
        );
        let result = hexdump(f1.clone(), len, 0, false);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), len);
        len = 20;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            len, len
        );
        let result = hexdump(f1.clone(), len, 0, false);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), len);
        len = 40;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            len, len
        );
        let result = hexdump(f1.clone(), len, 0, false);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), len);
        len = 990;
        const MAX_LEN: usize = 976;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            MAX_LEN, MAX_LEN
        );
        // Special case where we try and read past the end of the file
        let result = hexdump(f1.clone(), len, 0, false);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), MAX_LEN);
        let f2 = "test1.bin".to_string();
        len = 0x320;
        let offset = 0x2e0;
        println!(
            "hd {} bytes, visually inspect and make sure lines 300 & 310 are skipped",
            len
        );
        // Special case where we try and read past the end of the file
        let result = hexdump(f2.clone(), len, offset, false);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), len - offset);
    }

    #[test]
    fn test_hd_print_all_lines() {
        use crate::hexdump;
        let f1 = "test1.bin".to_string();
        let mut len = 10;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            len, len
        );
        let result = hexdump(f1.clone(), len, 0, true);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), len);

        len = 0x320;
        let offset = 0x2e0;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            len, len
        );
        let result = hexdump(f1.clone(), len, offset, true);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), len - offset);
    }

    #[test]
    fn test_hd_bytes_to_dump_and_offset() {
        use crate::hexdump;
        let f1 = "test0.bin".to_string();
        let mut len = 10;
        let mut offset = 5;
        println!("hd {} bytes starting at {}", len, offset);
        assert_eq!(hexdump(f1.clone(), len, offset, false).is_ok(), true);
        len = 20;
        offset = 1;
        println!("hd {} bytes starting at {}", len, offset);
        assert_eq!(hexdump(f1.clone(), len, offset, false).is_ok(), true);
        len = 40;
        offset = 14;
        println!("hd {} bytes starting at {}", len, offset);
        assert_eq!(hexdump(f1.clone(), len, offset, false).is_ok(), true);
    }

    #[test]
    fn test_hd_offset_eq_bytes_to_dump() {
        use crate::hexdump;
        let f1 = "test0.bin".to_string();
        assert_eq!(hexdump(f1.clone(), 10, 10, false).is_err(), true);
        assert_eq!(hexdump(f1.clone(), 20, 20, false).is_err(), true);
        assert_eq!(hexdump(f1.clone(), 0, 0, false).is_err(), true);
    }

    #[test]
    fn test_hd_offset_gt_bytes_to_dump() {
        use crate::hexdump;
        let f1 = "test0.bin".to_string();
        assert_eq!(hexdump(f1.clone(), 10, 10, true).is_err(), true);
        assert_eq!(hexdump(f1.clone(), 10, 11, false).is_err(), true);
        assert_eq!(hexdump(f1.clone(), 20, 2000, false).is_err(), true);
        assert_eq!(hexdump(f1.clone(), 0, 0xa1, false).is_err(), true);
    }
}
