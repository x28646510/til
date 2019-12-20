use std::io::{self};

fn main() {
    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).unwrap();
    let mut numbers_input = String::new();
    io::stdin().read_line(&mut numbers_input).unwrap();
    print!("{}", calc(count_input.as_str(), numbers_input.as_str()));
}

fn get_numbers(s: &str) -> Vec<i32> {
    let v: Vec<&str> = s.trim_end_matches('\n').split(" ").collect();
    v.into_iter().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn calc(count_input: &str, numbers_input: &str) -> i32 {
    let count = count_input.trim_end_matches('\n').parse::<usize>().unwrap();
    let mut alice = 0;
    let mut bob = 0;
    let mut numbers = get_numbers(numbers_input);
    numbers.sort_by(|a, b| b.cmp(a));
    for i in 0..count {
        match i % 2 {
            0 => alice += numbers[i],
            1 => bob += numbers[i],
            _ => (),
        }
    }

    alice - bob
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_1() {
        assert_eq!(calc("2", "3 1"), 2);
    }
    #[test]
    fn test_calc_2() {
        assert_eq!(calc("3", "2 7 4"), 5);
    }
    #[test]
    fn test_calc_3() {
        assert_eq!(calc("4", "20 18 2 18"), 18);
    }
}
