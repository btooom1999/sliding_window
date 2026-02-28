fn min_window(s: String, t: String) -> String {
    let mut s_bytes = vec![0; 58];
    let mut t_bytes = vec![0; 58];

    let mut need = 0;
    for byte in t.as_bytes() {
        let k = (byte-b'A') as usize;
        t_bytes[k] += 1;
        if t_bytes[k] == 1 {
            need += 1;
        }
    }

    let s = s.as_bytes();
    let mut window = None;
    let mut l = 0;
    let mut amount = 0;
    for r in 0..s.len() {
        let k = (s[r]-b'A') as usize;
        s_bytes[k] += 1;
        if s_bytes[k] == t_bytes[k] {
            amount += 1;
        }

        while amount == need {
            let k = (s[l]-b'A') as usize;
            s_bytes[k] -= 1;
            if s_bytes[k] < t_bytes[k] {
                amount -= 1;
            }

            if let Some((ol, or)) = window {
                if or-ol > r-l+1 {
                    window = Some((l, r+1));
                }
            } else {
                window = Some((l, r+1));
            }

            l += 1;
        }
    }

    if let Some((l, r)) = window {
        String::from_utf8(s[l..r].into()).unwrap()
    } else {
        String::new()
    }
}

pub fn main() {
    let s = "bba".to_string();
    let t = "ab".to_string();
    println!("{}", min_window(s, t));
}
