use std::collections::VecDeque;

fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut window = VecDeque::new();
    let n = colors.len();

    for r in 0..n + 2 {
        let i = if r >= n { r - n } else { r };
        window.push_back(colors[i]);

        if r > 2 {
            window.pop_front();
        }

        if r >= 2 && window[1] != window[0] && window[1] != window[2] {
            count += 1;
        }
    }

    count
}

pub fn main() {
    let colors = [0,1,0,0,1].to_vec();
    println!("{}", number_of_alternating_groups(colors));
}
