fn rotate(s: &str, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s.to_string();
    }

    let shift = (n % len + len) % len; // Обробка негативних значень та значень, що перевищують довжину рядка
    let mut result = String::new();

    if shift == 0 {
        return s.to_string(); // Якщо зсув дорівнює 0, повертаємо оригінальний рядок
    }

    let chars: Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        let new_index = (i as isize + len - shift) % len;
        result.push(chars[new_index as usize]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        for (n, exp) in shifts.iter() {
            assert_eq!(rotate(s, *n), *exp);
        }
    }
}

fn main() {
    let s = "abcdefgh";
    let test_shifts = [1, 2, -1, -2, 8, -8, 10, -10];

    for &n in test_shifts.iter() {
        println!("rotate(\"{}\", {}) = \"{}\"", s, n, rotate(s, n));
    }
}
