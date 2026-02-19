fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut l = 0;
    let mut cost = 0;
    let mut res = 0;
    for r in 0..s.len() {
        cost += (s[r] as i32 - t[r] as i32).abs();
        while cost > max_cost {
            cost -= (s[l] as i32 - t[l] as i32).abs();
            l += 1;
        }

        res = res.max(r - l + 1);
    }

    res as i32
}

pub fn main() {
    let s = "abcd".to_string();
    let t = "bcdf".to_string();
    let max_cost = 3;
    println!("{}", equal_substring(s, t, max_cost));
}
