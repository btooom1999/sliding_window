fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();

    let mut total = n as i32 * (n as i32 + 1) / 2;
    let mut result = 1;
    let mut l = 0;

    for r in 0..n {
        result *= nums[r];
        while l <= r && result >= k {
            total -= (n - r) as i32;
            result /= nums[l];
            l += 1;
        }
    }

    total
}

pub fn main() {
    let nums = [10,5,2,6].to_vec();
    let k = 100;
    println!("{}", num_subarray_product_less_than_k(nums, k));
}
