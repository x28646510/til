use std::io::{self};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    print!("{}", calc(line.as_str()));
}

fn calc(input: &str) -> &str {
    return judge(product(input));
}

fn product(input: &str) -> i32 {
    let vec: Vec<&str> = input.trim_end_matches('\n').split(" ").collect();
    return vec[0].parse::<i32>().unwrap() * vec[1].parse::<i32>().unwrap();
}

fn judge<'a>(value: i32) -> &'a str {
    if value % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_even() {
        assert_eq!(calc("3 4"), "Even");
    }

    fn test_calc_odd() {
        assert_eq!(calc("1 21"), "Odd");
    }
}
