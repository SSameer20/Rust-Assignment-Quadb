fn main(){
	let text = "Hi welcome to the world";
	let mut ans = text;

	for word in text.split(" ") {
		if word.len() < ans.len(){
			ans = word;
		}
	}
	println!("The shortest word is : {}",ans);
}