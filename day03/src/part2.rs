use std::cmp::Ordering;

fn max_val(line: &str) -> (usize, usize) {
    let v = line
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c as usize - '0' as usize))
        .max_by(|(i1, c1), (i2, c2)| match c1.cmp(c2) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => i2.cmp(i1),
        });

    println!("{}: {:?}", line, &v);

    v.unwrap()
}

fn voltage(line: &str) -> usize {
    let mut start = 0;
    let mut res = 0;

    for i in 0..12 {
        let (idx, num) = max_val(&line[start..line.len() - (12 - i - 1)]);

        start += idx + 1;
        res = res * 10 + num;
    }

    println!("{} -> {}", line, res);

    res
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

    assert_eq!(process(input), 3121910778619);
}

#[test]
fn test_voltage() {
    assert_eq!(voltage("987654321111111"), 987654321111);
}

#[test]
fn test_voltage2() {
    assert_eq!(voltage("818181911112111"), 888911112111);
}

#[test]
fn max_value() {
    assert_eq!(max_val("818181811112111"), (0, 8));
}
