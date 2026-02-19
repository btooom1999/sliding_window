fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let &max = nums.iter().max().unwrap();
    let mut max_count = 0;
    let mut l = 0;
    let mut res = 0;
    for (r, &num) in nums.iter().enumerate() {
        if num == max {
            max_count += 1;
        }

        while max_count >= k {
            if nums[l] == max {
                max_count -= 1;
            }
            l += 1;
        }

        res += l as i64;
    }

    res
}

pub fn main() {
    let nums = [1,3,2,3,3].to_vec();
    let k = 2;
    println!("{}", count_subarrays(nums, k));
}
