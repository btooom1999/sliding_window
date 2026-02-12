use std::collections::VecDeque;

fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = colors.len();

    let mut res = 0;
    let mut window = VecDeque::new();
    let mut duplicate = VecDeque::new();
    let mut duplicate_count = 0;
    for i in 0..n+k-1 {
        let x = i % n;
        if let Some(&idx) = window.back() && (idx + 1) % n == x && colors[idx] == colors[x] {
            duplicate.push_back(idx);
            duplicate_count += 1;
        }

        window.push_back(x);

        if i >= k && let Some(idx) = window.pop_front() && duplicate.front().is_some_and(|&v| v == idx) {
            duplicate.pop_front();
            duplicate_count -= 1;
        }

        if i >= k - 1 && duplicate_count == 0 {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let colors = [0,1,0,0,1,0,1].to_vec();
    let k = 6;
    println!("{}", number_of_alternating_groups(colors, k));
}
