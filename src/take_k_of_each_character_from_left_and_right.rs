fn take_characters(s: String, k: i32) -> i32 {
    let mut hashmap1 = [0;3];
    let s = s.as_bytes();
    for &b in s {
        hashmap1[(b-b'a') as usize] += 1;
    }

    if hashmap1.iter().any(|v| *v < k) {
        return -1;
    }

    let mut res = s.len() as i32;
    let mut hashmap2 = [0;3];

    let mut j = s.len() - 1;
    for i in (0..s.len()).rev() {
        let i = (s[i] - b'a') as usize;
        hashmap1[i] -= 1;
        while hashmap1[0] + hashmap2[0] < k || hashmap1[1] + hashmap2[1] < k || hashmap1[2] + hashmap2[2] < k {
            hashmap2[(s[j] - b'a') as usize] += 1;
            j -= 1;
        }

        res = res.min(hashmap1.iter().sum::<i32>() + hashmap2.iter().sum::<i32>());
    }

    res
}

pub fn main() {
    let s = "cbbac".to_string();
    let k = 1;
    println!("{}", take_characters(s, k));
}
