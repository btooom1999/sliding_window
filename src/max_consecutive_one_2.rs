fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut index_of_zero = 0;
    let mut found_zero = false;
    let mut res = 0;
    for (r, &num) in nums.iter().enumerate() {
        if found_zero && num == 0 {
            l = index_of_zero + 1;
            index_of_zero = r;
        }

        if !found_zero && num == 0 {
            found_zero = true;
            index_of_zero = r;
        }

        res = res.max((r - l + 1) as i32);
    }

    res
}

pub fn main() {
    // let nums = [1,0,1,1,0].to_vec();
    let nums = [1,0,1,1,0,1].to_vec();
    println!("{}", find_max_consecutive_ones(nums));
}
