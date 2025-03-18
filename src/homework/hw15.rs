use std::collections::HashSet;

fn solve_rebus() {
    let mut solve = 0;
    for m in 1..=8 {
        for u in 1..=8 {
            for h in 1..=8 {
                for a in 1..=8 {
                    for s in 1..=8 {
                        for l in 1..=8 {
                            for o in 1..=8 {
                                for n in 1..=8 {
                                    let mut digits = HashSet::new();
                                    digits.insert(m);
                                    digits.insert(u);
                                    digits.insert(h);
                                    digits.insert(a);
                                    digits.insert(s);
                                    digits.insert(l);
                                    digits.insert(o);
                                    digits.insert(n);

                                    if digits.len() == 8 {
                                        let muha = m * 1000 + u * 100 + h * 10 + a;
                                        let slon = s * 1000 + l * 100 + o * 10 + n;

                                        if muha * a == slon {
                                            println!("Розв'язок знайдено:");
                                            println!("  {}{}{}{}", m, u, h, a);
                                            println!("*");
                                            println!("              {}", a);
                                            println!("----------------");
                                            println!("  {}{}{}{}", s, l, o, n);
                                            println!();
                                            solve += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if solve != 0 {
        println!("Кількість розв'язків: {}", solve);
    } else {
        println!("Розв'язок не знайдено.");
    }
}

fn main() {
    solve_rebus();
                                      }
