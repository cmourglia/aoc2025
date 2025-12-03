fn is_invalid(id: usize) -> bool {
    let id = id.to_string();
    let id = id.as_bytes();

    let split = id.len() / 2;

    for i in 1..=split {
        if id.len() % i != 0 {
            continue;
        }

        let mut ok = true;
        let n_iter = id.len() - i;
        for j in 0..n_iter {
            let k = i + j;

            if k >= id.len() {
                break;
            }

            if id[j] != id[k] {
                ok = false;
                break;
            }
        }

        if ok {
            return true;
        }
    }

    return false;
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
                .map(|v| {
                    if is_invalid(v) {
                        println!("{v}");
                        v
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[test]
fn test() {
    let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    assert_eq!(process(input), 4174379265);
}

#[test]
fn test_id() {
    assert_eq!(is_invalid(55), true);
    assert_eq!(is_invalid(6464), true);
    assert_eq!(is_invalid(6463), false);
    assert_eq!(is_invalid(123123), true);
    assert_eq!(is_invalid(1234), false);
    assert_eq!(is_invalid(73289), false);
    assert_eq!(is_invalid(11111111), true);
    assert_eq!(is_invalid(12341234), true);
    assert_eq!(is_invalid(123123123), true);
    assert_eq!(is_invalid(12121212), true);
}

#[test]
fn incorrect_1() {
    assert_eq!(is_invalid(1188511880), false);
}

#[test]
fn incorrect_2() {
    assert_eq!(is_invalid(15415415), false);
}
