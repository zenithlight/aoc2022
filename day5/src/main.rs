// Based on https://fasterthanli.me/series/advent-of-code-2022/part-5

use std::{env, fmt};

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while1};
use nom::IResult;
use nom::sequence::{delimited, preceded, tuple};
use nom::combinator::{all_consuming, map, map_res, opt};
use nom::Finish;

#[derive(Clone)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let label_parser = delimited(tag("["), take(1_usize), tag("]"));
    let crate_from_label = |s: &str| Crate(s.chars().next().unwrap());

    map(label_parser, crate_from_label)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_number),
            preceded(tag(" to "), parse_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src: src - 1, dst: dst - 1 }
    )(i)
}

fn transpose_rev<T>(rows: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!rows.is_empty());

    let len = rows[0].len();

    // turn the vector into a vector of iterators, from which we will take one item at a time
    // collecting them into a new vector.
    let mut iters: Vec<_> = rows.into_iter().map(|row| row.into_iter()).collect();

    (0..len).map(|_| {
        iters
            .iter_mut()
            .rev()
            .filter_map(|row| row.next().unwrap())
            .collect()
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", a(input));

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", b(input));
}

fn a(input: String) -> String {
    let mut lines = input.split('\n').into_iter();

    let crate_lines: Vec<_> = (&mut lines).map_while(|line| {
        all_consuming(parse_crate_line)(line).finish().ok().map(|(_, line)| line)
    }).collect();

    // our result is by rows, but we need stacks, so transpose the Vec<Vec<Option<Crate>>>
    let mut crate_stacks = transpose_rev(crate_lines);

    lines.next(); // consume separator

    for instruction in lines
        .filter_map(|line| all_consuming(parse_instruction)(line).finish().ok())
        .map(|instruction| instruction.1) {
        for _ in 0..instruction.quantity {
            let c = crate_stacks[instruction.src].pop().unwrap();
            crate_stacks[instruction.dst].push(c);
        }
    }

    crate_stacks
        .iter()
        .filter_map(|stack| stack.last())
        .map(|c| c.0)
        .collect::<String>()
}

fn b(input: String) -> String {
    let mut lines = input.split('\n').into_iter();

    let crate_lines: Vec<_> = (&mut lines).map_while(|line| {
        all_consuming(parse_crate_line)(line).finish().ok().map(|(_, line)| line)
    }).collect();

    // our result is by rows, but we need stacks, so transpose the Vec<Vec<Option<Crate>>>
    let mut crate_stacks = transpose_rev(crate_lines);

    lines.next(); // consume separator

    for instruction in lines
        .filter_map(|line| all_consuming(parse_instruction)(line).finish().ok())
        .map(|instruction| instruction.1) {
        let source_length = crate_stacks[instruction.src].len();
        // could use `get_many_mut` to avoid heap allocation from `collect`
        let substack = crate_stacks[instruction.src].drain(source_length - instruction.quantity..).collect_vec();
        crate_stacks[instruction.dst].extend(substack)
    }

    crate_stacks
        .iter()
        .filter_map(|stack| stack.last())
        .map(|c| c.0)
        .collect::<String>()
}

#[test]
fn test_example_a() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(a(input), "CMZ");
}

#[test]
fn test_example_b() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(b(input), "MCD");
}