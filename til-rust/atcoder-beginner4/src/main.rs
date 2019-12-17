use std::io::{self};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).unwrap();
    print!("{}", count(n.as_str(), numbers.as_str()));
}

fn count(n: &str, numbers: &str) -> i32 {
    let m = n.trim_end_matches('\n').parse::<i32>().unwrap();
    let sv: Vec<&str> = numbers.trim_end_matches('\n').split(" ").collect();
    let mut iv: Vec<i32> = sv
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .rev()
        .collect();
    let mut count = 0;
    loop {
        iv = match check(m, iv) {
            Some(new_numbers) => {
                count += 1;
                new_numbers
            }
            None => break,
        }
    }

    return count;
}

fn check(m: i32, numbers: Vec<i32>) -> Option<Vec<i32>> {
    let mut new_numbers: Vec<i32> = vec![];
    for n in numbers {
        if n % 2 == 1 {
            return None;
        }
        new_numbers.push(n / 2)
    }
    return Some(new_numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_1() {
        assert_eq!(count("3", "8 12 40"), 2);
    }

    fn test_count2() {
        assert_eq!(count("4", "5 6 8 10"), 0);
    }

    fn test_count3() {
        assert_eq!(
            count(
                "6",
                "382253568 723152896 37802240 379425024 404894720 471526144"
            ),
            8
        );
    }
}
