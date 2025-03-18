use rand::{Rng, rng};

fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total_weight: u32 = shipments.iter().sum();
    let num_ships = shipments.len();

    if total_weight % num_ships as u32 != 0 {
        return usize::MAX; // Або -1, якщо ви хочете повернути -1
    }

    let average_weight = total_weight / num_ships as u32;
    let mut moves = 0;

    for &shipment in shipments {
        if shipment > average_weight {
            moves += shipment - average_weight;
        }
    }

    moves as usize
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rng();
    let average_weight = rng.random_range(1..10) as u32; // Випадкова середня вага
    let mut total_weight = 0;
    let mut shipments = Vec::with_capacity(n);

    for _ in 0..n - 1 {
        let shipment = rng.random_range(1..average_weight * 2); // Випадкова вага
        shipments.push(shipment);
        total_weight += shipment;
    }

    let last_shipment = average_weight * n as u32 - total_weight;
    shipments.push(last_shipment);

    shipments
}

#[test]
fn main() {
    let n = 5; // Кількість кораблів
    let shipments = gen_shipments(n);

    println!("Ваги вантажу на кораблях: {:?}", shipments);

    let moves = count_permutation(&shipments);
    if moves == usize::MAX {
        println!("Неможливо розподілити вантаж однаково.");
    } else {
        println!("Мінімальна кількість переміщень: {}", moves);
    }
}
