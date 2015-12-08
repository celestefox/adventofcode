use std::collections::HashMap;

fn main() {
    let directions = include_str!("input.txt");
    println!("{:?}", deliver_presents("^v", 1));
    println!("One santa: {} houses", deliver_presents(directions, 1).len());
    println!("Two santas: {} houses", deliver_presents(directions, 2).len());
}

struct Santa {
    x: i32,
    y: i32
}

impl Santa {
    fn new(x: i32, y:i32) -> Santa {
        Santa { x: x, y: y }
    }
}

fn deliver_presents(directions: &str, num_santas: usize) -> HashMap<(i32, i32), u32> {
    let mut santas = Vec::new();
    for i in 0..num_santas {
        santas.push(Santa::new(0, 0));
    }
    debug_assert!(santas.len() == num_santas);
    let mut houses = HashMap::new();
    for (santa, direction) in (0..num_santas).cycle().zip(directions.chars()) {
        match direction {
            '<' => santas[santa].x -= 1,
            '>' => santas[santa].x += 1,
            'v' => santas[santa].y -= 1,
            '^' => santas[santa].y += 1,
            _ => panic!("Invalid direction")
        }
        *houses.entry((santas[santa].x, santas[santa].y)).or_insert(0) += 1;
    }
    houses
}
