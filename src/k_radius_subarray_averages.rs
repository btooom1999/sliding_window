fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let max_window = k*2+1;
    let mut res = vec![-1; nums.len()];

    let mut l = 0;
    let mut sum = 0;
    for (r, num) in nums.iter().enumerate() {
        sum += *num as i64;
        if r >= max_window {
            sum -= nums[l] as i64;
            l += 1;
        }

        if r >= max_window - 1 {
            res[r-k] = (sum / max_window as i64) as i32;
        }
    }

    res
}

pub fn main() {
    let nums = [7,4,3,9,1,8,5,2,6].to_vec();
    let k = 3;
    println!("{:?}", get_averages(nums, k));
}
