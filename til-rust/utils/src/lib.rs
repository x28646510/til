use std::io::{self};

pub fn readline(mut input: String) -> String {
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn parse_all<T>(s: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let v: Vec<&str> = s.split(" ").collect();
    v.into_iter().map(|x| parse(x)).collect()
}

pub fn parse<T>(s: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.trim_end_matches('\n').parse::<T>().unwrap()
}

pub fn check_duplicate<T>(mut v: Vec<T>, n: T) -> Vec<T>
where
    T: std::clone::Clone + std::cmp::PartialEq,
{
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

    #[test]
    fn test_check_duplicate3() {
        assert_eq!(
            check_duplicate(vec!["1", "2", "3"], "2"),
            vec!["1", "2", "3"]
        )
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse::<i32>("123"), 123)
    }
}
