#[inline]
fn reverse_str(s: &str) -> String
{
    s.chars().rev().collect::<String>()
}

fn rotate_right(s: &str, d: usize) -> String
{
    let s1 = reverse_str(&s[..d]);
    let s2 = reverse_str(&s[d..]);
    reverse_str(&(s1 + &s2))
}

fn main() {
    let s = "Hello, world!";
    println!("{} -> {} ; {}", &s, reverse_str(&s), rotate_right(&s, 4));
}
