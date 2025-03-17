fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

fn main() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
        (1231, false),
    ];

    for (n, exp) in data.iter() {
        println!("is_palindrome({}) -> {} (очікувано: {})", n, is_palindrome(*n), exp);
    }
}
