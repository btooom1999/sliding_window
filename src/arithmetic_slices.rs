fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut count = 1;
    let mut k = 0;
    let mut res = 0;
    for i in 1..nums.len() {
        if nums[i-1] + k != nums[i] {
            count = 2;
            k = nums[i] - nums[i-1];
        } else {
            count += 1;
        }

        res += std::cmp::max(count - 2, 0);
    }

    res
}

pub fn main() {
    let nums = [1,2,3,4].to_vec();
    println!("{}", number_of_arithmetic_slices(nums));
}
