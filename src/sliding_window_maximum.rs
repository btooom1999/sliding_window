fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut max = Vec::new();;

    for (r, &num) in nums.iter().enumerate() {
        while let Some(val) = max.last() {

        }

        if r >= k {

        }
    }
}

pub fn main() {
    let nums = [1,3,-1,-3,5,3,6,7].to_vec();
    let k = 3;
    println!("{}", max_sliding_window(nums, k));
}
