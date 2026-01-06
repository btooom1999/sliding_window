fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() < needle.len() {
        return -1;
    }

    let haystack = haystack.chars().collect::<Vec<_>>();
    let mut needle_windows = vec![];
    let mut haystack_windows = vec![];

    for (i, c) in needle.chars().enumerate() {
        needle_windows.push(c);
        haystack_windows.push(haystack[i]);
    }

    if needle_windows == haystack_windows {
        return 0;
    }

    for (i, c) in haystack.iter().enumerate().skip(needle.len()) {
        haystack_windows.remove(0);
        haystack_windows.push(*c);

        if haystack_windows == needle_windows {
            return (i - needle.len() + 1) as i32;
        }
    }

    -1
}

pub fn main() {
    let haystack = "mississippi".to_string();
    let needle = "pi".to_string();
    println!("{}", str_str(haystack, needle));
}


