fn main() {
    let s = "Hi from Sameer";
    let revseredString = string_reverse(s);

    println!("The Original String is : {}", s);
    println!("The Original String is : {}", revseredString);

}

fn string_reverse(text: &str) -> String {
  text.chars().rev().collect()
}
