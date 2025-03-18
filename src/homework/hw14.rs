fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::from("")];
    }

    let mut result = Vec::new();
    result.push(String::from("0"));
    result.push(String::from("1"));

    for i in 2..(1 << n) {
        if i & (i - 1) == 0 {
            let mut reversed = result.clone();
            reversed.reverse();

            for j in 0..result.len() {
                result[j] = String::from("0") + &result[j];
                reversed[j] = String::from("1") + &reversed[j];
            }

            result.extend(reversed);
        }
    }

    result
}

// таблиця змінена по даним з інтернету, та таблиця яка була тут не співпадала з інформацією з різних сайтів
#[test]
fn test() {
    let test_data =
        [
            (0, vec!("")),
            (1, vec!("0", "1")),
            (2, vec!("00", "01", "11", "10")),
            (3, vec!("000", "001", "011", "010",
                     "110", "111", "101", "100")),
            (4, vec!("0000", "0001", "0011", "0010",
                     "0110", "0111", "0101", "0100",
                     "1100", "1101", "1111", "1110",
                     "1010", "1011", "1001", "1000")),

        ];


    test_data
        .iter()
        .for_each(|(n, out)|
            assert_eq!(gray(*n), *out)
        );
}


