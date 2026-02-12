use std::collections::VecDeque;

fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let mut window = VecDeque::with_capacity(k as usize +1);
    let mut sum = 0;
    let mut res = 0;

    for (i, &num) in arr.iter().enumerate() {
        let i = i as i32 + 1;
        window.push_back(num);
        sum += num;

        if i > k {
            sum -= window.pop_front().unwrap();
        }

        if i >= k {
            res += (sum / k >= threshold) as i32;
        }
    }

    res
}

pub fn main() {
    let arr = [2,2,2,2,5,5,5,8].to_vec();
    let k = 3;
    let threshold = 4;
    println!("{}", num_of_subarrays(arr, k, threshold));
}
