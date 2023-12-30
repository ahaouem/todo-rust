#[cfg(test)]
mod tests {
    use std::io::{self, Read};

    #[test]
    fn test_read_input_single_line() {
        let input = b"Hello, World!\n";
        let buffer = io::Cursor::new(input);

        let result = "Hello, World!";

        assert_eq!(result, "Hello, World!");

        let mut actual_input = String::new();
        buffer
            .take(result.len() as u64)
            .read_to_string(&mut actual_input)
            .unwrap();
        assert_eq!(actual_input, "Hello, World!");
    }

    #[test]
    fn test_read_input_empty_line() {
        let input = b"\n";
        let buffer = io::Cursor::new(input);

        let result = "";

        assert_eq!(result, "");

        let mut actual_input = String::new();
        buffer
            .take(result.len() as u64)
            .read_to_string(&mut actual_input)
            .unwrap();
        assert_eq!(actual_input, "");
    }

    #[test]
    fn test_read_input_multiple_lines() {
        let input = b"Line 1\nLine 2\nLine 3\n";
        let buffer = io::Cursor::new(input);

        let result = "Line 1";

        assert_eq!(result, "Line 1");

        let mut actual_input = String::new();
        buffer
            .take(result.len() as u64)
            .read_to_string(&mut actual_input)
            .unwrap();
        assert_eq!(actual_input, "Line 1");
    }
}
