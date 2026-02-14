use std::collections::{HashMap, VecDeque};

fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;

    let mut hashmap = HashMap::new();
    let mut errors = VecDeque::new();

    let mut l = 0;
    let mut res = 0;
    let mut sum = 0;

    for (r, &num) in nums.iter().enumerate() {
        sum += num as i64;
        let val = hashmap.entry(num).or_insert(0);
        *val += 1;
        if *val > 1 {
            errors.push_back(num);
        }

        if r >= k {
            *hashmap.get_mut(&nums[l]).unwrap() -= 1;
            sum -= nums[l] as i64;
            if let Some(&error) = errors.front() && error == nums[l] {
                errors.pop_front();
            }

            l += 1;
        }

        if r >= k - 1 && errors.is_empty() {
            res = res.max(sum);
        }
    }

    res
}

pub fn main() {
    let nums = [9,9,9,1,2,3,1,5,4,9,9,9].to_vec();
    let k = 3;
    println!("{}", maximum_subarray_sum(nums, k));
}
