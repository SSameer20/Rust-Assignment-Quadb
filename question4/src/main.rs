fn main() {
 
  let num = 9;


    fn prime_num(n : i32) -> bool {
        if n == 2 {
            return true;
        }
        for i in 2 ..n {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }

    let x = prime_num(num);

    if x {
        println!("The {} is Prime Number", num);
    }
    else{
        println!("The {} is not a Prime Number", num);
    }
}
