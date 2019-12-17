use std::io::{self};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    print!("{}", count(input.as_str()));
}

fn count(line: &str) -> i32 {
    let mut num = 0;
    for char in line.chars() {
        match char {
            '1' => num += 1,
            _ => {}
        }
    }
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_1() {
        assert_eq!(count("101"), 2);
    }

    fn test_count_2() {
        assert_eq!(count("000"), 0);
    }
}
