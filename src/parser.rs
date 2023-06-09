use std::{num::NonZeroU16, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit0, digit1, multispace0, one_of},
    error::ParseError,
    multi::many_m_n,
    sequence::{delimited, pair, tuple},
    Finish, IResult,
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
    let (rest, (adv, dis, mul, max)) = ws(tuple((
        many_m_n(0, 1, one_of("gх")),
        many_m_n(0, 1, one_of("bп")),
        digit0,
        many_m_n(0, 1, pair(one_of("dк"), digit1)),
    )))(input)?;
    let max = max.first().map_or("1", |(_, max)| max);
    let advantage = match adv.len().cmp(&dis.len()) {
        std::cmp::Ordering::Less => Some(false),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(true),
    };

    Ok((
        rest,
        Roll::Dices {
            mul: NonZeroU16::from_str(mul).map_or(1, |value| value.into()),
            max: NonZeroU16::from_str(max).unwrap().into(),
            advantage,
        },
    ))
}

fn sum(input: &str) -> IResult<&str, Roll> {
    let (rest, (left, _, right)) = ws(tuple((dices, ws(tag("+")), alt((sum, dices)))))(input)?;
    Ok((rest, Roll::Sum(Box::new(left), Box::new(right))))
}

pub fn roll(input: &str) -> Result<(&str, Roll), nom::error::Error<&str>> {
    alt((sum, dices))(input).finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dices() {
        assert_eq!(
            dices("1d6"),
            Ok((
                "",
                Roll::Dices {
                    mul: 1,
                    max: 6,
                    advantage: None
                }
            ))
        );
        assert_eq!(
            dices("2d9"),
            Ok((
                "",
                Roll::Dices {
                    mul: 2,
                    max: 9,
                    advantage: None
                }
            ))
        );
        assert_eq!(
            dices("16"),
            Ok((
                "",
                Roll::Dices {
                    mul: 16,
                    max: 1,
                    advantage: None
                }
            ))
        );
    }
}
