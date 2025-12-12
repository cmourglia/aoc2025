use std::thread::available_parallelism;

use glam::*;
use itertools::Itertools;

pub fn process(src: &str) -> u64 {
    let points = src
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>();

    let lines = points
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&(i64, i64), &(i64, i64))>>();

    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            (
                a,
                b,
                (i64::abs_diff(a.0, b.0) + 1) * (i64::abs_diff(a.1, b.1) + 1),
            )
        })
        .sorted_by_key(|a| a.2)
        .rev()
        .find(|(a, b, area)| {
            lines.iter().all(|(line_start, line_end)| {
                let left_of_rect = a.0.max(b.0) <= line_start.0.min(line_end.0);
                let right_of_rect = a.0.min(b.0) >= line_start.0.max(line_end.0);
                let above = a.1.max(b.1) <= line_start.1.min(line_end.1);
                let below = a.1.min(b.1) >= line_start.1.max(line_end.1);

                left_of_rect || right_of_rect || above || below
            })
        })
        .unwrap()
        .2 as u64
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

    assert_eq!(process(input), 24);
}

#[test]
fn test_a() {
    let points = vec![
        (7, 1),
        (11, 1),
        (11, 7),
        (9, 7),
        (9, 5),
        (2, 5),
        (2, 3),
        (7, 3),
    ];
    let lines = points
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&(i32, i32), &(i32, i32))>>();

    let a = (2, 1);
    let b = (7, 3);

    let res = lines.iter().all(|(line_start, line_end)| {
        // if line is to left
        let left_of_rect = a.0.max(b.0) <= line_start.0.min(line_end.0);
        let right_of_rect = a.0.min(b.0) >= line_start.0.max(line_end.0);
        let above = a.1.max(b.1) <= line_start.1.min(line_end.1);
        let below = a.1.min(b.1) >= line_start.1.max(line_end.1);

        left_of_rect || right_of_rect || above || below
    });

    assert!(true);
}
