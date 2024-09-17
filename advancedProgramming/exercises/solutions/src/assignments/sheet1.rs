// Sheet 1 exercises

pub fn string_reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn string_reverse_2(s: &str) -> String {
    let mut s_rev = String::new();
    for c in s.chars().rev() {
        s_rev.push(c);
    }
    s_rev
}