use rand::{Rng, rng};

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rng();
    (0..n).map(|_| rng.random_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, usize, usize)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_index1 = 0;
    let mut min_index2 = 1;

    for i in 1..data.len() - 1 {
        let current_sum = data[i] + data[i + 1];
        if current_sum < min_sum {
            min_sum = current_sum;
            min_index1 = i;
            min_index2 = i + 1;
        }
    }

    Some((min_sum, min_index1, min_index2))
}

fn print_vector_info(data: &[i32]) {
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:4}", i)).collect::<Vec<_>>().join(""));
    println!("data:    {}", data.iter().map(|&x| format!("{:4}", x)).collect::<Vec<_>>().join(""));

    if let Some((_, index1, index2)) = min_adjacent_sum(data) {
        let mut indexes_line = String::new();
        for i in 0..data.len() {
            if i == index1 || i == index2 {
                if i==index1{
                    indexes_line.push_str("  \\__");
                }else if i==index2{
                    indexes_line.push_str("__/");
                }
            } else {
                indexes_line.push_str("    ");
            }
        }
        println!("indexes: {}", indexes_line);

        println!("min adjacent sum={}+{}={} at indexes: {},{}", data[index1], data[index2], data[index1] + data[index2], index1, index2);
    } else {
        println!("min adjacent sum: немає достатньо елементів");
    }
}
#[test]
fn main() {
    let data = gen_random_vector(20);
    print_vector_info(&data);
}
