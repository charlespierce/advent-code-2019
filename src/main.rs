fn main() {
    let mut valid_count = 0;
    for i in 172_851..675_870 {
        if is_valid(i) {
            valid_count += 1;
        }
    }

    println!("Valid Passwords: {}", valid_count);
}

fn is_valid(input: u32) -> bool {
    let input_str = input.to_string();
    let digits = input_str.chars();
    let mut has_double = false;
    let mut prev_char: Option<char> = None;

    for chr in digits {
        if let Some(prev) = prev_char {
            if chr < prev {
                return false;
            }
            if chr == prev {
                has_double = true;
            }
        }

        prev_char = Some(chr);
    }

    has_double
}