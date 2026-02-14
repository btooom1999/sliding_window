fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut k = 1;
    let mut l = 0;
    let mut res = 0;
    for (r, &num) in nums.iter().enumerate() {
        if num == 0 {
            k -= 1;
        }

        while k == -1 {
            if nums[l] == 0 {
                k = 0;
            }

            l += 1;
        }

        res = res.max(r - l);
    }

    res as i32
}

pub fn main() {
    let nums = [0,1,1,1,0,1,1,0,1].to_vec();
    println!("{}", longest_subarray(nums));
}
