use utils::readline;

fn main() {
    println!(
        "{}",
        calc(readline(String::new()).as_str().trim_end_matches('\n'))
    );
}

fn calc(s: &str) -> &str {
    if is_valid(s) {
        return "YES";
    }
    return "NO";
}

fn is_valid(s: &str) -> bool {
    if process_d(s) {
        return true;
    }
    if process_e(s) {
        return true;
    }
    false
}

fn process_d(mut s: &str) -> bool {
    if !s.starts_with("dream") {
        return false;
    }
    s = &s[5..s.len()];
    if s == "" {
        return true;
    }
    if s == "er" {
        return true;
    }
    if s.starts_with("d") {
        return process_d(s);
    }
    if s.starts_with("erd") {
        return process_d(&s[2..s.len()]);
    }
    if s.starts_with("ere") {
        return process_e(&s[2..s.len()]);
    }
    if s.starts_with("era") {
        return process_e(s);
    }
    false
}

fn process_e(mut s: &str) -> bool {
    if !s.starts_with("erase") {
        return false;
    }
    s = &s[5..s.len()];
    if s == "" {
        return true;
    }
    if s == "r" {
        return true;
    }
    if s.starts_with("d") {
        return process_d(s);
    }
    if s.starts_with("e") {
        return process_e(s);
    }
    if s.starts_with("rd") {
        return process_d(&s[1..s.len()]);
    }
    if s.starts_with("re") {
        return process_e(&s[1..s.len()]);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid1() {
        assert_eq!(is_valid("erasedream"), true)
    }

    #[test]
    fn test_is_valid2() {
        assert_eq!(is_valid("dreameraser"), true)
    }

    #[test]
    fn test_is_valid3() {
        assert_eq!(is_valid("dreamerer"), false)
    }
}
