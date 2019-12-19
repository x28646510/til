use std::io::{self};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (n, a, b) = get_numbers(input.as_str());
    print!("{}", calc(n, a, b));
}

fn get_numbers(s: &str) -> (i32, i32, i32) {
    let v: Vec<&str> = s.trim_end_matches('\n').split(" ").collect();
    let vv: Vec<i32> = v.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    (vv[0], vv[1], vv[2])
}

fn get_digit_sum(n: i32) -> i32 {
    let mut m = 1;
    let mut sum = 0;
    loop {
        let digit_num = n / m;
        if digit_num == 0 {
            break;
        }
        let prev = m;
        m *= 10;
        sum += (n % m) / prev
    }
    sum
}

fn calc(n: i32, a: i32, b: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        let digit_sum = get_digit_sum(i);
        if a <= digit_sum && digit_sum <= b {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digit_sum_1() {
        assert_eq!(get_digit_sum(14), 5)
    }

    #[test]
    fn test_get_digit_sum_2() {
        assert_eq!(get_digit_sum(256), 13)
    }

    #[test]
    fn test_get_digit_sum_3() {
        assert_eq!(get_digit_sum(5256), 18)
    }

    #[test]
    fn test_calc_1() {
        assert_eq!(calc(20, 2, 5), 84)
    }

    #[test]
    fn test_calc_2() {
        assert_eq!(calc(10, 1, 2), 13)
    }

    #[test]
    fn test_calc_3() {
        assert_eq!(calc(100, 4, 16), 4554)
    }
}
