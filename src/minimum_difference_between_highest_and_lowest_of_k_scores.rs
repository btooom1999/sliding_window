use std::collections::VecDeque;

fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();

    let mut window = VecDeque::new();
    let mut res = i32::MAX;

    for (i, &num) in nums.iter().enumerate() {
        let i = i as i32 + 1;
        window.push_back(num);

        if i > k {
            window.pop_front();
        }

        if i >= k {
            res = res.min(window.back().unwrap() - window.front().unwrap());
        }
    }

    res
}

pub fn main() {
    let nums = [9,4,1,7].to_vec();
    // 1 4 7 9
    let k = 2;
    println!("{}", minimum_difference(nums, k));
}
