fn main() {
  let s = "WoW1";
  let flag = pal(s);
  if flag {
	  println!("{} is palindrome", s);
  }
  else{
	  println!("{} is not palindrome", s);
  }
  fn pal(text: &str) -> bool {
	let n = text.len();
	for i in 0..n{
		let left = text.chars().nth(i).unwrap_or(' ');
		let right = text.chars().nth(n - i - 1).unwrap_or(' ');

		if left != right {
			return false;
		}
	}
	
	return true;
 }


  
  
	 
}

