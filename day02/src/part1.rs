fn is_invalid(id: usize) -> bool {
    let id = id.to_string();

    if id.len() % 2 == 1 {
        return false;
    }

    let split = id.len() / 2;

    for i in 0..split {
        let j = i + split;

        if id.chars().nth(i) != id.chars().nth(j) {
            return false;
        }
    }

    return true;
}

pub fn process(src: &str) -> usize {
    src.split(',')
        .map(|range| {
            range
                .split('-')
                .map(|s| s.trim().parse::<usize>().unwrap_or(0))
                .map(|v| v)
                .collect::<Vec<_>>()
        })
        .map(|range| {
            (range[0]..=range[1])
                .map(|v| if is_invalid(v) { v } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[test]
fn test() {
    let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    assert_eq!(process(input), 1227775554);
}

#[test]
fn test_id() {
    assert_eq!(is_invalid(55), true);
    assert_eq!(is_invalid(6464), true);
    assert_eq!(is_invalid(123123), true);
    assert_eq!(is_invalid(1234), false);
    assert_eq!(is_invalid(73289), false);
}
