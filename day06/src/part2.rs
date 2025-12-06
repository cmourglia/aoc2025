use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    multi::{many1, separated_list1},
};

pub fn process(src: &str) -> u64 {
    let transposed = transpose(src);
    let numbers = parse_nums(&transposed);
    let (_, ops) = parse_ops(transposed.last().unwrap()).unwrap();

    let mut sum = 0;

    for (i, column) in numbers.iter().enumerate() {
        let op = ops[i];
        let mut col_result = match op {
            Op::Add => 0,
            Op::Mul => 1,
        };

        for n in column {
            col_result = op.op(col_result, *n)
        }

        sum += col_result;
    }

    sum
}

fn transpose(src: &str) -> Vec<String> {
    let lines: Vec<&str> = src.lines().collect();

    let cols = lines[0].chars().count();

    let mut out = vec![String::with_capacity(lines.len()); cols + 1];

    for line in &lines[..lines.len() - 1] {
        for (i, c) in line.chars().enumerate() {
            out[i].push(c);
        }
    }

    out.push(String::from(*lines.last().unwrap()));

    out
}

fn parse_nums(input: &Vec<String>) -> Vec<Vec<u64>> {
    let mut res = vec![];

    let mut current_row = vec![];

    for num in &input[..input.len() - 1] {
        match num.trim_ascii().parse::<u64>() {
            Ok(n) => current_row.push(n),
            Err(_) => {
                res.push(current_row.clone());
                current_row.clear();
            }
        }
    }

    res
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn op(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

fn parse_ops(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(
        many1(tag(" ")),
        alt((tag("*"), tag("+"))).map(|c| match c {
            "*" => Op::Mul,
            "+" => Op::Add,
            _ => panic!("invalid char {}", c),
        }),
    )
    .parse(input)
}

#[test]
fn test() {
    let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    assert_eq!(process(input), 3263827);
}

#[test]
fn test_transpose() {
    let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    assert_eq!(
        transpose(input),
        vec![
            "1  ",
            "24 ",
            "356",
            "   ",
            "369",
            "248",
            "8  ",
            "   ",
            " 32",
            "581",
            "175",
            "   ",
            "623",
            "431",
            "  4",
            "",
            "*   +   *   +  "
        ]
    );
}

#[test]
fn test_parse_nums() {
    let input = vec![
        "1  ".to_string(),
        "24 ".to_string(),
        "356".to_string(),
        "   ".to_string(),
        "369".to_string(),
        "248".to_string(),
        "8  ".to_string(),
        "   ".to_string(),
        " 32".to_string(),
        "581".to_string(),
        "175".to_string(),
        "   ".to_string(),
        "623".to_string(),
        "431".to_string(),
        "  4".to_string(),
        "".to_string(),
        "*   +   *   +  ".to_string(),
    ];

    assert_eq!(
        parse_nums(&input),
        vec![
            vec![1, 24, 356],
            vec![369, 248, 8],
            vec![32, 581, 175],
            vec![623, 431, 4]
        ]
    );
}

//r#"
//123 328  51 64
// 64 64  387 23
//  6 98  215 314
//*   +   *   +
//
//1  *
//26
//346
//
//369+
//248
//8
//
// 32*
//581
//175
//
//623+
//431
//  4
//"#;
