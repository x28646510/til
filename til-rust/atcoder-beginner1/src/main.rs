use std::io::{self};

fn main() {
  let mut a = String::new();
  io::stdin().read_line(&mut a).unwrap();
  let mut bc = String::new();
  io::stdin().read_line(&mut bc).unwrap();
  let bcvec: Vec<&str> = bc.trim_end_matches('\n').split(" ").collect();
  let sum = a.trim_end_matches('\n').parse::<i32>().unwrap() + bcvec[0].parse::<i32>().unwrap() + bcvec[1].parse::<i32>().unwrap();
  let mut s = String::new();
  io::stdin().read_line(&mut s).unwrap();
  print!("{} {}", sum, s);
}
