use std::io::{self};

fn main() {
    let a = String::new();
    let b = String::new();
    let c = String::new();
    let s = String::new();
    let an = readline(a);
    let bn = readline(b);
    let cn = readline(c);
    let sum = readline(s);
    print!("{}", calc(an, bn, cn, sum));
}

fn readline(mut s: String) -> i32 {
    io::stdin().read_line(&mut s).unwrap();
    s.trim_end_matches('\n').parse::<i32>().unwrap()
}

fn create_combinations(a: i32, b: i32, c: i32) -> Vec<(i32, i32, i32)> {
    let mut v: Vec<(i32, i32, i32)> = vec![];
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                v.push((i, j, k));
            }
        }
    }

    v
}

fn calc(a: i32, b: i32, c: i32, sum: i32) -> i32 {
    let mut count = 0;
    for (i, j, k) in create_combinations(a, b, c) {
        if 500 * i + 100 * j + 50 * k == sum {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test_calc_1() {
        assert_eq!(calc(2, 2, 2, 100), 2)
    }

    #[test]
    fn test_calc_2() {
        assert_eq!(calc(5, 1, 0, 150), 0)
    }

    #[test]
    fn test_calc_3() {
        assert_eq!(calc(30, 40, 50, 6000), 213)
    }
}
