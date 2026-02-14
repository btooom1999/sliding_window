use std::collections::HashMap;

fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut hashmap = HashMap::new();
    let mut l = 0;
    let mut res = 0;
    let mut flag = false;

    for (r, &num) in nums.iter().enumerate() {
        let val = hashmap.entry(num).or_insert(0);
        *val += 1;
        if *val > k {
            flag = true;
        }

        while flag && let Some(prev_val) = hashmap.get_mut(&nums[l]) {
            if nums[l] == num {
                flag = false;
            }
            *prev_val -= 1;
            l += 1;
        }

        res = res.max(r - l + 1);
    }

    res as i32
}

pub fn main() {
    let nums = [1,4,4,3].to_vec();
    let k = 1;
    println!("{}", max_subarray_length(nums, k));
}
