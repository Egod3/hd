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
}
