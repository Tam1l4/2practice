fn main() {
    let num_triangles = 8;
    let max_width = 2 * num_triangles - 1;

    for n in 1..=num_triangles {
        for i in 0..n {
            let stars = "*".repeat((2 * i + 1) as usize);
            let spaces = " ".repeat(((max_width - stars.len()) / 2) as usize);
            println!("{}{}", spaces, stars);
        }
    }
}
