pub mod one {

    /// Returns the calibration code from the input string, unsafely
    /// panicking if the input does not contain a digit.
    fn digits(puzzle_line: &String) -> u32 {
        let filtered_chars: String = puzzle_line
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .collect();
        let first: char = filtered_chars
            .chars()
            .next()
            .unwrap_or_else(|| panic!("Failed to find a digit."));
        let last: char = filtered_chars
            .chars()
            .last()
            .unwrap_or_else(|| panic!("Failed to find a digit."));
        let value: u32 = format!("{}{}", first, last).parse::<u32>().unwrap();
        value
    }

    pub fn solution(lines: Vec<String>) {
        let result: u32 = lines.iter().map(|l| digits(l)).sum();
        println!("Decoded the calibration code. Result: {}", result);
    }
}
