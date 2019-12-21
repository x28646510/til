use std::io::{self};

fn main() {
    let mut v = vec![];
    let count = readline();
    for _ in 0..count {
        let n = readline();
        v = check_duplicate(v, n);
    }
    print!("{}", v.len());
}

fn readline() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .as_str()
        .trim_end_matches('\n')
        .parse::<i32>()
        .unwrap()
}

fn check_duplicate(mut v: Vec<i32>, n: i32) -> Vec<i32> {
    if !v.contains(&n) {
        v.push(n);
    }
    v.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_duplicate1() {
        assert_eq!(check_duplicate(vec![1, 2, 3], 2), vec![1, 2, 3])
    }

    #[test]
    fn test_check_duplicate2() {
        assert_eq!(check_duplicate(vec![1, 2, 3], 4), vec![1, 2, 3, 4])
    }
}
