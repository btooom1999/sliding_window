use std::collections::VecDeque;

fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();

    let k = k as i64;

    let mut window = VecDeque::new();
    let mut sum = 0;
    let mut res = 0;

    for (r, &num) in nums.iter().enumerate() {
        sum += num as i64;
        window.push_back(num);

        while nums[r] as i64 * window.len() as i64 > sum + k {
            sum -= window.pop_front().unwrap() as i64;
        }

        res = res.max(window.len());
    }

    res as i32
}

pub fn main() {
    let nums = [1,2,4].to_vec();
    let k = 5;
    println!("{}", max_frequency(nums, k));
}
