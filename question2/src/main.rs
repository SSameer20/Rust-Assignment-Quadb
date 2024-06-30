fn main() {
    let array : [i32; 5] = [10, 20, 30, 30, 40];
    let mut ans: i32 = -1;
    let target = 30;
    for i in 0..5 {
        if array[i] == target{
            ans = i as i32;
            break;
        }

    }
    println!("The {} is first occured at position {}", target, ans);
}
