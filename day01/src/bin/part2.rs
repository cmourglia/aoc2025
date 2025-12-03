fn main() {
    let file = include_str!("../../input.txt");
    let result = day01::part2::process(file);
    println!("{}", result);
}
