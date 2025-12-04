use std::cmp::Ordering;

fn voltage(line: &str) -> usize {
    let (idx, c1) = line[0..line.len() - 1]
        .chars()
        .enumerate()
        .max_by(|(i1, c1), (i2, c2)| match c1.cmp(c2) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => i2.cmp(i1),
        })
        .unwrap();

    let c2 = line[idx + 1..].chars().max().unwrap();

    10 * (c1 as usize - '0' as usize) + (c2 as usize - '0' as usize)
}

pub fn process(src: &str) -> usize {
    src.lines().map(|l| voltage(l)).sum::<usize>()
}

#[test]
fn test() {
    let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    assert_eq!(process(input), 357);
}

#[test]
fn test_voltage() {
    assert_eq!(voltage("987654321111111"), 98);
}

#[test]
fn test_voltage2() {
    assert_eq!(voltage("818181811112111"), 88);
}
