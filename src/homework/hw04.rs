fn main() {
    const H: u32 = 11;
    const W: u32 = 11;
    let x_cent = W as i32 / 2;
    let y_cent = H as i32 / 2;

    for y in 0..H as i32 {
        for x in 0..W as i32 {
            let dist_x = (x - x_cent).abs();
            let dist_y = (y - y_cent).abs();

            let is_inside = dist_x + dist_y <= y_cent;

            let sum = if is_inside { "*" } else { " " };
            print!("{}", sum);
        }
        println!();
    }
}
