fn main() {
    let boxes = include_str!("input.txt");
    let boxes : Vec<Vec<u32>> = boxes.lines().map(|line| line.split('x').map(|side| side.parse().expect("Side is not a number")).collect()).collect();
    let mut paper_total = 0;
    let mut ribbon_total = 0;
    for mut boxx in boxes {
        boxx.sort();
        paper_total += 3*(boxx[0]*boxx[1]) + 2*(boxx[1]*boxx[2]) + 2*(boxx[2]*boxx[0]);
        ribbon_total += 2*boxx[0]+2*boxx[1] + boxx[0]*boxx[1]*boxx[2];
    }
    println!("Total paper needed is {} sq. ft.", paper_total);
    println!("Total ribbon needed is {} ft.", ribbon_total);
}
