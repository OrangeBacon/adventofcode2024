mod day1;

fn main() {
    let inp = std::fs::read_to_string("data/day1.txt").unwrap();
    println!("{}", day1::part1(&inp));
    println!("{}", day1::part2(&inp));
}
