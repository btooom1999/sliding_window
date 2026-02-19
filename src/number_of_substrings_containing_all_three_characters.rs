fn number_of_substrings(s: String) -> i32 {
    let s = s.into_bytes();
    let n = s.len() - 1;

    let mut hashmap = [0;3];
    let mut l = 0;
    let mut res = 0;
    for (r, &b) in s.iter().enumerate() {
        hashmap[(b - b'a') as usize] += 1;
        while hashmap.iter().all(|v| *v > 0) {
            hashmap[(s[l] - b'a') as usize] -= 1;
            l += 1;
        }

        res += l;
    }

    res as i32
}

pub fn main() {
    let s = "abcabc".to_string();
    println!("{}", number_of_substrings(s));
}
