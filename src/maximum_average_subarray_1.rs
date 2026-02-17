fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut sum = 0_f64;
    let mut l = 0;
    let mut res = f64::MIN;
    for r in 0..nums.len() {
        sum += nums[r] as f64;
        if r >= k {
            sum -= nums[l] as f64;
            l += 1;
        }

        if r >= k - 1 {
            res = res.max(sum / k as f64);
        }
    }

    res
}

pub fn main() {
    let nums = [1,12,-5,-6,50,3].to_vec();
    let k = 4;
    println!("{}", find_max_average(nums, k));
}
