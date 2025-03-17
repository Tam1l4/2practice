fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else if c.is_lowercase() {
                c.to_uppercase().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}

#[test]
fn test_invert_the_case() {
    let data = [
        ("Hello", "hELLO"),
        ("Метелик", "мЕТЕЛИК"),
    ];

    data.iter().for_each(|(a, b)| {
        assert_eq!(invert_the_case(a.to_string()), *b);
        assert_eq!(invert_the_case(b.to_string()), *a);
    });
}

fn main() {
    let text = "Метелик Hello Rust!";
    let inverted = invert_the_case(text.to_string());
    
    println!("Оригінал: {}", text);
    println!("Змінений регістр: {}", inverted);
}
