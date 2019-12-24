use utils::{parse, parse_all, readline};

fn main() {
    let count = parse::<i32>(readline(String::new()).as_str());
    let mut p = Point::new(0, 0, 0);
    let mut ret = "Yes";
    for _ in 0..count {
        let v = parse_all::<i32>(readline(String::new()).as_str());
        if ret == "No" {
            continue;
        }
        let t = v[0];
        let x = v[1];
        let y = v[2];
        match p.forward(t, x, y) {
            Some(_) => (),
            None => {
                ret = "No";
            }
        }
    }
    print!("{}", ret);
}

struct Point {
    t: i32,
    x: i32,
    y: i32,
}

impl Point {
    fn new(t: i32, x: i32, y: i32) -> Self {
        Point { t, x, y }
    }

    fn can_forward(&self, t: i32, x: i32, y: i32) -> bool {
        let dx = (self.x - x).abs();
        let dy = (self.y - y).abs();
        let rest = (t - self.t) - (dx + dy);
        if rest < 0 {
            return false;
        }
        if rest % 2 == 1 {
            return false;
        }
        true
    }

    fn forward(&mut self, t: i32, x: i32, y: i32) -> Option<()> {
        if self.can_forward(t, x, y) {
            self.t = t;
            self.x = x;
            self.y = y;
            return Some(());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_forward1() {
        let p = Point::new(0, 0, 0);
        assert_eq!(p.can_forward(3, 1, 2), true)
    }

    #[test]
    fn test_can_forward2() {
        let p = Point::new(3, 1, 2);
        assert_eq!(p.can_forward(6, 1, 1), true)
    }

    #[test]
    fn test_can_forward3() {
        let p = Point::new(0, 0, 0);
        assert_eq!(p.can_forward(2, 100, 100), false)
    }

    #[test]
    fn test_can_forward4() {
        let p = Point::new(0, 0, 0);
        assert_eq!(p.can_forward(5, 1, 1), false)
    }
}
