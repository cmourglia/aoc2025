use std::ops::RangeInclusive;

pub fn process(src: &str) -> usize {
    let mut process_ingredients = false;

    let mut ranges: Vec<RangeInclusive<usize>> = vec![];

    let mut fresh = 0;

    for line in src.lines() {
        if line.is_empty() {
            process_ingredients = true;
            continue;
        }

        if process_ingredients {
            let ingredient = line.parse::<usize>().unwrap();

            for r in &ranges {
                if r.contains(&ingredient) {
                    fresh += 1;
                    break;
                }
            }
        } else {
            let v = line
                .split("-")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let range = v[0]..=v[1];
            ranges.push(range);
        }
    }

    fresh
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

    assert_eq!(process(input), 3);
}
