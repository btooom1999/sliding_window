fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();

    let mut res = 0;
    let mut l = 0;
    for r in 0..nums.len() {
        while nums[r] - nums[l] > 2 * k {
            l += 1;
        }

        res = res.max(r - l + 1);
    }

    res as i32
}

pub fn main() {
    let nums = [4,6,1,2].to_vec();
    let k = 2;
    println!("{}", maximum_beauty(nums, k));
}
