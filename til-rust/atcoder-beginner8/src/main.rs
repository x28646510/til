use std::io::{self};
use utils::{check_duplicate, parse, readline};

fn main() {
    let mut v = vec![];
    let count = parse::<i32>(readline(String::new()).as_str());
    for _ in 0..count {
        let n = parse::<i32>(readline(String::new()).as_str());
        v = check_duplicate(v, n);
    }
    print!("{}", v.len());
}
