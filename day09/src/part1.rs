use glam::*;
use itertools::Itertools;

pub fn process(src: &str) -> u64 {
    src.lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .tuple_combinations()
        .map(|(a, b)| (i64::abs_diff(a[0], b[0]) + 1) * (i64::abs_diff(a[1], b[1]) + 1))
        .sorted_by(|a, b| b.partial_cmp(a).unwrap())
        .take(1)
        .next()
        .unwrap() as u64
}

#[test]
fn test() {
    let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    assert_eq!(process(input), 50);
}
