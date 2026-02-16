fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut count = 0;
    let mut res = 0;
    for &num in &nums {
        if num == 0 {
            count += 1;
        } else {
            count = 0;
        }

        res += count;
    }

    res
}

pub fn main() {
    let nums = [1,3,0,0,2,0,0,4].to_vec();
    println!("{}", zero_filled_subarray(nums));
}
