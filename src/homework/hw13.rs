struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied_points = std::collections::HashSet::new();

    for rect in xs {
        let min_x = rect.a.x.min(rect.b.x);
        let max_x = rect.a.x.max(rect.b.x);
        let min_y = rect.a.y.min(rect.b.y);
        let max_y = rect.a.y.max(rect.b.y);

        for x in min_x..max_x {
            for y in min_y..max_y {
                occupied_points.insert((x, y));
            }
        }
    }

    occupied_points.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}
#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
