fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut l = 0;
    let mut res = Vec::new();
    let mut error = -1;

    for (r, &num) in nums.iter().enumerate() {
        if r > 0 && nums[r-1] + 1 != num {
            error = r as i32 - 1;
        }

        if r >= k as usize {
            if l == error {
                error = -1;
            }
            l += 1;
        }

        if r >= k as usize - 1 {
            if error == -1 {
                res.push(num);
            } else {
                res.push(-1);
            }
        }
    }

    res
}

pub fn main() {
    let nums = [3,2,3,2,3,2].to_vec();
    let k = 2;
    println!("{:?}", results_array(nums, k));
}
