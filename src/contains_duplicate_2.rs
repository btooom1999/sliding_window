use std::collections::HashSet;

fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut hashset = HashSet::new();
    let mut l = 0;
    for (r, num) in nums.iter().enumerate() {
        if r - l > k as usize {
            hashset.remove(&nums[l]);
            l += 1;
        }
        if hashset.contains(num) {
            return true;
        }
        hashset.insert(*num);
    }

    false
}

pub fn main() {
    let nums = [0,0,0].to_vec();
    let k = 0;
    println!("{}", contains_nearby_duplicate(nums, k));
}
