use crate::util::read_inputs;
use anyhow::{anyhow, Result};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{anychar, char, i32};
use nom::combinator::{eof, opt, peek, value};
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::{IResult, Parser};

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Op {
    None,
    Mul(i32, i32),
}

impl Op {
    fn result(&self) -> i32 {
        match self {
            Op::None => 0,
            Op::Mul(a, b) => a * b,
        }
    }
}

fn parse_mul(i: &str) -> IResult<&str, Op> {
    delimited(tag("mul("), separated_pair(i32, char(','), i32), char(')'))(i)
        .map(|(remaining, (a, b))| (remaining, Op::Mul(a, b)))
}

fn parse_disabled(i: &str) -> IResult<&str, Op> {
    value(
        Op::None, // Output is thrown away.
        delimited(tag("don't()"), take_until("do()"), tag("do()")),
    )(i)
}

fn parse_eof(i: &str) -> IResult<&str, Op> {
    value(Op::None, eof).parse(i)
}

fn skip_disabled(i: &str) -> IResult<&str, ()> {
    let (tail, _) = many_till(anychar, peek(alt((parse_disabled, parse_mul, parse_eof))))(i)?;
    Ok((tail, ()))
}

fn parse_instructions(i: &str) -> IResult<&str, Vec<Op>> {
    let (tail, _) = opt(skip_disabled)(i)?;

    let (tail, r) = many0(terminated(
        alt((parse_disabled, parse_mul)),
        opt(skip_disabled),
    ))(tail)?;

    Ok((
        tail,
        r.iter().filter(|&&op| op != Op::None).cloned().collect(),
    ))
}

fn skip_not_mul(i: &str) -> IResult<&str, ()> {
    let (tail, _) = many_till(anychar, peek(alt((parse_mul, parse_eof))))(i)?;
    Ok((tail, ()))
}

fn parse_instructions_no_disabled(i: &str) -> IResult<&str, Vec<Op>> {
    // advance until first instruction
    let (tail, _) = opt(skip_not_mul)(i)?;

    // parse out pairs of instruction+noise
    let (tail, ops) = many0(terminated(parse_mul, opt(skip_not_mul)))(tail)?;

    Ok((tail, ops))
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(3)?;
    let joined = inputs.join("");
    let mut result: i32;

    let (_, ops) = parse_instructions_no_disabled(joined.as_str())
        .map_err(|e| anyhow!("Failed to parse {e}"))?;
    result = ops.iter().map(|&op| op.result()).sum();

    println!("Day 3 Part 1: {result}");

    // ---------------------------------------

    let (_, ops) =
        parse_instructions(joined.as_str()).map_err(|e| anyhow!("Failed to parse {e}"))?;
    result = ops.iter().map(|&op| op.result()).sum();

    println!("Day 3 Part 2: {result}");

    Ok(())
}
