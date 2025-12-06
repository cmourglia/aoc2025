use std::ops::RangeInclusive;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{self, line_ending},
    combinator::map_res,
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
};

use itertools::Itertools;

pub fn process(src: &str) -> u64 {
    let (_, (numbers, ops)) = parse(src).unwrap();

    let numbers: Vec<Vec<u64>> = (0..numbers[0].len())
        .map(|i| numbers.iter().map(|row| row[i]).collect())
        .collect();

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

fn parse(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Op>)> {
    separated_pair(parse_numbers, line_ending, parse_ops).parse(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        many0(tag(" ")),
        separated_list1(many1(tag(" ")), complete::u64),
        many0(tag(" ")),
    )
    .parse(input)
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

    assert_eq!(process(input), 4277556);
}

#[test]
fn test_line() {
    assert_eq!(
        parse_line(" 123 328  61    64\n"),
        Ok(("\n", vec![123, 328, 61, 64]))
    );
}
