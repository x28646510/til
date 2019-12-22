use utils::{parse_all, readline};

fn main() {
    let v = parse_all::<i32>(readline(String::new()).as_str());
    let count = v[0];
    let total = v[1];
    let (a, b, c) = calc(count, total);
    print!("{} {} {}", a, b, c);
}

fn calc(count: i32, total: i32) -> (i32, i32, i32) {
    for b in 0..=count {
        for c in 0..=(count - b) {
            let candidate = 10000 * (count - b - c) + 5000 * b + 1000 * c;
            if candidate == total {
                return (count - b - c, b, c);
            }
        }
    }
    return (-1, -1, -1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_1() {
        assert_eq!(calc(9, 45000), (4, 0, 5))
    }

    #[test]
    fn test_calc_2() {
        assert_eq!(calc(20, 196000), (-1, -1, -1))
    }

    #[test]
    fn test_calc_3() {
        assert_eq!(calc(1000, 1234000), (26, 0, 974))
    }

    #[test]
    fn test_calc_4() {
        assert_eq!(calc(2000, 20000000), (2000, 0, 0))
    }
}
