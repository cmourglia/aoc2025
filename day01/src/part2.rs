pub fn process(src: &str) -> i64 {
    let mut index = 50;
    let mut zeroes = 0;

    for line in src.lines() {
        let sign = match line.chars().next() {
            Some('R') => 1,
            Some('L') => -1,
            _ => 0,
        };

        let val = line[1..].parse::<i64>().unwrap();

        if index == 0 && sign < 0 {
            index = 100;
        }

        for _i in 0..val {
            index = index + sign * 1;

            if index == 0 {
                zeroes += 1;
                index = 100;
            } else if index == 100 {
                zeroes += 1;
                index = 0;
            }
        }

        index = (index + 100) % 100;
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

    assert_eq!(process(input), 6);
}

#[test]
fn test_1000() {
    let input = r#"R1000"#;
    assert_eq!(process(input), 10);
}

#[test]
fn test_1000_2() {
    let input = r#"R50
R1000"#;
    assert_eq!(process(input), 11);
}
