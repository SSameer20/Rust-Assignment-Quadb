fn main() {
    let num = 10;
    fn check_prime_num(n : &i32) -> bool {
        for i in 2..*n{
            if *n == 2 {
                return true;
            }
            if *n % i == 0{
                return false;
            }
        }
        return true;
    }
    let flag = check_prime_num(&num);
    if flag {
        println!("The Number {} is Prime", num);
    }
    else{
        println!("The Number {} is Not Prime", num);
    }
}
