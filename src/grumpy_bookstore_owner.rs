use std::collections::VecDeque;

fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let n = customers.len();
    let mut prefix = vec![0; n];
    for i in 0..n {
        let prev_prefix = if i == 0 { 0 } else { prefix[i-1] };
        prefix[i] = if grumpy[i] == 1 { 0 } else { customers[i] } + prev_prefix;
    }

    let mut window = VecDeque::new();
    let mut res = 0;
    let mut sum = 0;
    let mut l = 0;
    for (r, &num) in customers.iter().enumerate() {
        let r = r as i32 + 1;
        window.push_back(num);
        sum += num;

        if r > minutes {
            sum -= window.pop_front().unwrap();
            l += 1;
        }

        if r >= minutes {
            let prefix_before = if r == minutes { 0 } else { prefix[l-1] };
            let prefix_last = prefix[n-1] - prefix[r as usize-1];
            res = res.max(prefix_before + sum + prefix_last);
        }
    }


    res
}

pub fn main() {
    let customers = vec![10,1,7];
    let grumpy = vec![0,0,0];
    let minutes = 2;
    println!("{}", max_satisfied(customers, grumpy, minutes));
}
