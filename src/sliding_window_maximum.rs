use std::collections::VecDeque;

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as _;
    let mut res = Vec::new();
    let mut stack = VecDeque::<(i32, i32)>::new();
    let mut l = 0;

    for (r, &num) in nums.iter().enumerate() {
        let mut count = 1;
        while let Some(&last) = stack.back() && last.0 < num  {
            count += stack.pop_back().unwrap().1;
        }

        stack.push_back((num, count));

        if r >= k {
            let front = stack.pop_front().unwrap();
            if front.1 > 1 {
                stack.push_front((front.0, front.1-1));
            }
            l += 1;
        }

        if r >= k-1 {
            res.push(stack.front().unwrap().0);
        }
    }

    res
}

pub fn main() {
    let nums = [1,3,-1,-3,5,3,6,7].to_vec();
    let k = 3;
    println!("{:?}", max_sliding_window(nums, k));
}
