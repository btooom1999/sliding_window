fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if s.len() < p.len() {
        return vec![];
    }

    let s = s.chars().collect::<Vec<_>>();
    let p_len = p.len();
    let mut p_windows = vec![0;26];
    let mut s_windows = vec![0;26];

    for (i, c) in p.chars().enumerate() {
        p_windows[(c as u8 - b'a') as usize] += 1;
        s_windows[(s[i] as u8 - b'a') as usize] += 1;
    }

    let mut res = Vec::new();
    if p_windows == s_windows {
        res.push(0);
    }

    for (i, _) in s.iter().enumerate().skip(p_len) {
        s_windows[(s[i - p_len] as u8 - b'a') as usize] -= 1;
        s_windows[(s[i] as u8 - b'a') as usize] += 1;

        if s_windows == p_windows {
            res.push((i - p_len + 1) as i32);
        }
    }

    res
}

pub fn main() {
    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    println!("{:?}", find_anagrams(s, p));
}
