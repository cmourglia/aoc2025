fn main() {
    let file = include_str!("../../input.txt");
    let result = day01::part1::process(file);
    println!("{}", result);
}
