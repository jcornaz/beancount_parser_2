use nom::{
    branch::alt,
    bytes::{complete::tag, complete::take_while},
    character::complete::{char, satisfy, space0},
    combinator::{cut, recognize},
    multi::{many1_count, separated_list0},
    sequence::{delimited, preceded},
};

use crate::currency::{self, Currency};

use super::{IResult, Span};

#[derive(Debug)]
pub struct Account<'a>(&'a str);

impl<'a> Account<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Open<'a> {
    pub account: Account<'a>,
    pub currencies: Vec<Currency<'a>>,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Close<'a> {
    pub account: Account<'a>,
}

pub(super) fn parse(input: Span<'_>) -> IResult<'_, Account<'_>> {
    let (input, name) = recognize(preceded(
        alt((
            tag("Expenses"),
            tag("Assets"),
            tag("Liabilities"),
            tag("Income"),
            tag("Equity"),
        )),
        cut(many1_count(preceded(
            char(':'),
            preceded(
                satisfy(|c: char| c.is_uppercase()),
                take_while(|c: char| c.is_alphanumeric() || c == '-'),
            ),
        ))),
    ))(input)?;
    Ok((input, Account(name.fragment())))
}

pub(super) fn open(input: Span<'_>) -> IResult<'_, Open<'_>> {
    let (input, account) = parse(input)?;
    let (input, _) = space0(input)?;
    let sep = delimited(space0, char(','), space0);
    let (input, currencies) = separated_list0(sep, currency::parse)(input)?;
    Ok((
        input,
        Open {
            account,
            currencies,
        },
    ))
}

pub(super) fn close(input: Span<'_>) -> IResult<'_, Close<'_>> {
    let (input, account) = parse(input)?;
    Ok((input, Close { account }))
}
