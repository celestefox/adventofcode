fn main() {
    let program = include_str!("input.txt");
    let mut floor: i32 = 0;
    for c in program.char_indices() {
        match c {
            (_, '(') => floor += 1,
            (_, ')') => floor -= 1,
            _ => panic!("Invalic character"),
        }
        match c {
            (step, _) => if floor == -1 {
                println!("Entered first basement on step {}", step + 1);
            },
        }
    }
    println!("{}", floor);
}
