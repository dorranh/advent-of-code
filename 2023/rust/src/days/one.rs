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

    /// Converts digits encoded in their English spelling in the input string
    /// to an encoding containing their numeric counterparts. This approach allows
    /// for overlapping strings such as oneight which should parse to both 1 and 8.
    fn make_digits(puzzle_line: &String) -> String {
        puzzle_line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
    }

    pub fn solution(lines: Vec<String>) {
        let result: u32 = lines.iter().map(|l| digits(&make_digits(l))).sum();
        println!("Decoded the calibration code. Result: {}", result);
    }
}
