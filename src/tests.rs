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
    fn test_hexdump_len() {
        use crate::hexdump;
        let f1 = "test0.bin".to_string();
        let mut len = 10;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            len, len
        );
        let result = hexdump(f1.clone(), len, 0);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), len);
        len = 20;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            len, len
        );
        let result = hexdump(f1.clone(), len, 0);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), len);
        len = 40;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            len, len
        );
        let result = hexdump(f1.clone(), len, 0);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), len);
        len = 990;
        const MAX_LEN: usize = 976;
        println!(
            "hd {} bytes, visually inspect and make sure only {} bytes are printed",
            MAX_LEN, MAX_LEN
        );
        // Special case where we try and read past the end of the file
        let result = hexdump(f1.clone(), len, 0);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), MAX_LEN);
    }

    //// TODO: Get this test to pass.
    //#[test]
    //fn test_hexdump_len_and_off() {
    //    use crate::hexdump;
    //    let f1 = "test0.bin".to_string();
    //    let mut len = 10;
    //    println!(
    //        "hd {} bytes, visually inspect and make sure only {} bytes are printed",
    //        len, len
    //    );
    //    assert_eq!(hexdump(f1.clone(), len, 5).is_ok(), true);
    //    len = 20;
    //    println!(
    //        "hd {} bytes, visually inspect and make sure only {} bytes are printed",
    //        len, len
    //    );
    //    assert_eq!(hexdump(f1.clone(), len, 1).is_ok(), true);
    //    len = 40;
    //    println!(
    //        "hd {} bytes, visually inspect and make sure only {} bytes are printed",
    //        len, len
    //    );
    //    assert_eq!(hexdump(f1.clone(), len, 14).is_ok(), true);
    //}

    #[test]
    fn test_hexdump_off_eq_len() {
        use crate::hexdump;
        let f1 = "test0.bin".to_string();
        assert_eq!(hexdump(f1.clone(), 10, 10).is_err(), true);
        assert_eq!(hexdump(f1.clone(), 20, 20).is_err(), true);
        assert_eq!(hexdump(f1.clone(), 0, 0).is_err(), true);
    }

    #[test]
    fn test_hexdump_off_gt_len() {
        use crate::hexdump;
        let f1 = "test0.bin".to_string();
        assert_eq!(hexdump(f1.clone(), 10, 11).is_err(), true);
        assert_eq!(hexdump(f1.clone(), 20, 2000).is_err(), true);
        assert_eq!(hexdump(f1.clone(), 0, 0xa1).is_err(), true);
    }
}
