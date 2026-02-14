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
            *prev_val -= 1;
            l += 1;
            if nums[l-1] == num {
                flag = false;
                break;
            }
        }

        res = res.max(r - l + 1);
    }

    res as i32
}

pub fn main() {
    let nums = [1,2,3,1,2,3,1,2].to_vec();
    let k = 2;
    println!("{}", max_subarray_length(nums, k));
}
