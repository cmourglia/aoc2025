use std::{collections::HashSet, ops::RangeInclusive};

use rstest::rstest;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Range(usize, usize);

fn overlaps(a: Range, b: Range) -> bool {
    let (la, ha) = (a.0, a.1);
    let (lb, hb) = (b.0, b.1);

    if la == lb || ha == hb || la == hb || lb == ha {
        return true;
    }

    if la > lb && la < hb {
        return true;
    }

    if lb > la && lb < ha {
        return true;
    }

    if ha > lb && ha < hb {
        return true;
    }

    if hb > la && hb < ha {
        return true;
    }

    false
}

pub fn merge_ranges(a: Range, b: Range) -> Range {
    let (la, ha) = (a.0, a.1);
    let (lb, hb) = (b.0, b.1);

    Range(usize::min(la, lb), usize::max(ha, hb))
}

pub fn process(src: &str) -> usize {
    let mut ranges: Vec<Range> = vec![];

    for line in src.lines() {
        if line.is_empty() {
            break;
        }

        let v = line
            .split("-")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        ranges.push(Range(v[0], v[1]));
    }

    loop {
        let mut one_overlap = false;
        let mut cleaned_ranges = vec![];

        cleaned_ranges.push(ranges[0]);

        for range in &ranges[1..] {
            let mut found = false;
            for clean_range in &mut cleaned_ranges {
                if overlaps(*range, *clean_range) {
                    *clean_range = merge_ranges(*clean_range, *range);
                    found = true;
                    one_overlap = true;
                    break;
                }
            }

            if !found {
                cleaned_ranges.push(*range);
            }
        }

        ranges = cleaned_ranges;

        ranges.sort_by(|k1, k2| k1.0.cmp(&k2.0));

        if !one_overlap {
            break;
        }
    }

    ranges.iter().map(|r| r.1 - r.0 + 1).sum::<usize>()
}

#[test]
fn test() {
    let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    assert_eq!(process(input), 14);
}

#[rstest]
#[case((Range(16, 20), Range(12, 18)), true)]
#[case((Range(12, 20), Range(13, 18)), true)]
#[case((Range(14, 15), Range(12, 18)), true)]
#[case((Range(1, 2), Range(4, 5)), false)]
#[case((Range(8, 9), Range(2, 4)), false)]
fn test_overlaps(#[case] input: (Range, Range), #[case] expected: bool) {
    assert_eq!(expected, overlaps(input.0, input.1));
}

#[rstest]
#[case((Range(16, 20), Range(12, 18)), Range(12, 20))]
#[case((Range(12, 20), Range(13, 18)), Range(12, 20))]
#[case((Range(14, 15), Range(12, 18)), Range(12, 18))]
fn test_merge(#[case] input: (Range, Range), #[case] expected: Range) {
    assert_eq!(expected, merge_ranges(input.0, input.1));
}
