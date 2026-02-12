use std::collections::VecDeque;

fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let mut window = VecDeque::new();
    let mut opt = 0;
    let mut res = i32::MAX;

    for (r, &num) in blocks.as_bytes().iter().enumerate() {
        let r = r as i32;
        window.push_back(num);
        if num == b'W' {
            opt += 1;
        }

        if r >= k && let Some(val) = window.pop_front() && val == b'W' {
            opt -= 1;
        }

        if r >= k - 1 {
            res = std::cmp::min(res, opt);
        }

    }

    res
}

pub fn main() {
    let blocks = "WWBBBWBBBBBWWBWWWB".to_string();
    let k = 16;
    println!("{}", minimum_recolors(blocks, k));
}
