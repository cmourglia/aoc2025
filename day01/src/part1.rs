pub fn process(src: &str) -> i64 {
    let mut index = 50;
    let mut zeroes = 0;

    for line in src.lines() {
        let sign = match line.chars().next() {
            Some('R') => 1,
            Some('L') => -1,
            _ => 0,
        };

        let val = sign * &line[1..].parse::<i64>().unwrap();

        index = (index + val + 100) % 100;

        if index == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

#[test]
fn test() {
    let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    assert_eq!(process(input), 3);
}
