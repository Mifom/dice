use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, one_of},
    error::ParseError,
    multi::many_m_n,
    sequence::{delimited, pair, tuple},
    IResult,
};

use crate::roll::Roll;

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn dices(input: &str) -> IResult<&str, Roll> {
    let (rest, (mul, vec)) = ws(pair(digit1, many_m_n(0, 1, pair(one_of("dк"), digit1))))(input)?;
    let max = vec.first().map_or("1", |(_, max)| max);

    Ok((
        rest,
        Roll::Dices {
            mul: u16::from_str(mul).unwrap(),
            max: u16::from_str(max).unwrap(),
        },
    ))
}

fn sum(input: &str) -> IResult<&str, Roll> {
    let (rest, (left, _, right)) = ws(tuple((dices, ws(tag("+")), alt((sum, dices)))))(input)?;
    Ok((rest, Roll::Sum(Box::new(left), Box::new(right))))
}

pub fn roll(input: &str) -> IResult<&str, Roll> {
    alt((sum, dices))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dices() {
        assert_eq!(dices("1d6"), Ok(("", Roll::Dices { mul: 1, max: 6 })));
        assert_eq!(dices("2d9"), Ok(("", Roll::Dices { mul: 2, max: 9 })));
        assert_eq!(dices("16"), Ok(("", Roll::Dices { mul: 16, max: 1 })));
    }
}
