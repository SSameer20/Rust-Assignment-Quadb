fn main() {
    let nums : [i32; 5] = [1,5,9,10,15];
    let n = nums.len();
    let mut ans = 0;
    if n % 2 != 0 {
        ans = nums[n / 2]; 
    }
    else{
        ans = (nums[n/2] + nums[n/2 - 1]) / 2;
    }

    println!("the Median of the Array is  {}", ans);
    
}
