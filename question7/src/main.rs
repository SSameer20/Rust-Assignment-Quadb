use std::slice;

fn main() {
  let mut numbers = [5, 2, 8, 1, 4];
  let k = 4;
  let mut ans = 0;
  numbers.sort();
  ans = numbers[k - 1];
  println!("The {} th smallest number is {} ", k, ans);
}