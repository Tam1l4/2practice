fn main() {
    const WIDTH: u32 = 30;
    const HEIGHT: u32 = 10;
    let k = WIDTH as f32 / HEIGHT as f32;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let row1 = y == 0;
            let row_n = y == HEIGHT - 1;
            let col1 = x == 0;
            let col_n = x == WIDTH - 1;
            let is_hor = row1 || row_n;
            let is_ver = col1 || col_n;
            let is_diag1 = x == (y as f32 * k).round() as u32;
            let is_diag2 = x == WIDTH - 1 - (y as f32 * k).round() as u32;
            let sum = if is_hor || is_ver || is_diag1 || is_diag2 {
                "*"
            } else {
                " "
            };

            print!("{}", sum);
        }
        println!();
    }
}
