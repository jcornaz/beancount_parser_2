use std::borrow::Borrow;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    sync::Arc,
};

use nom::{
    branch::alt,
    bytes::{complete::tag, complete::take_while},
    character::complete::{char, satisfy, space0, space1},
    combinator::{cut, iterator, opt, recognize},
    multi::many1_count,
    sequence::{delimited, preceded},
};

use crate::{
    amount::{self, Amount, Currency},
    Decimal,
};

use super::{IResult, Span};

/// Account
///
/// You may convert it into a string slice with [`Account::as_str`]
///
/// # Example
/// ```
/// use beancount_parser_2::DirectiveContent;
/// let input = "2022-05-24 open Assets:Bank:Checking";
/// let beancount = beancount_parser_2::parse::<f64>(input).unwrap();
/// let DirectiveContent::Open(open) = &beancount.directives[0].content else { unreachable!() };
/// assert_eq!(open.account.as_str(), "Assets:Bank:Checking");
/// ```
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Account(Arc<str>);

impl Account {
    /// Returns underlying string representation
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl AsRef<str> for Account {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Borrow<str> for Account {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

/// Open account directive
///
/// # Example
/// ```
/// use beancount_parser_2::DirectiveContent;
/// let input = "2022-05-24 open Assets:Bank:Checking    CHF";
/// let beancount = beancount_parser_2::parse::<f64>(input).unwrap();
/// let DirectiveContent::Open(open) = &beancount.directives[0].content else { unreachable!() };
/// assert_eq!(open.account.as_str(), "Assets:Bank:Checking");
/// assert_eq!(open.currencies.iter().next().unwrap().as_str(), "CHF");
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Open {
    /// Account being open
    pub account: Account,
    /// Currency constraints
    pub currencies: HashSet<Currency>,
}

/// Close account directive
///
/// # Example
/// ```
/// use beancount_parser_2::DirectiveContent;
/// let input = "2022-05-24 close Assets:Bank:Checking";
/// let beancount = beancount_parser_2::parse::<f64>(input).unwrap();
/// let DirectiveContent::Close(close) = &beancount.directives[0].content else { unreachable!() };
/// assert_eq!(close.account.as_str(), "Assets:Bank:Checking");
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Close {
    /// Account being closed
    pub account: Account,
}

/// Balance assertion
///
/// # Example
/// ```
/// use beancount_parser_2::DirectiveContent;
/// let input = "2022-05-24 balance Assets:Bank:Checking 10 CHF";
/// let beancount = beancount_parser_2::parse::<f64>(input).unwrap();
/// let DirectiveContent::Balance(balance) = &beancount.directives[0].content else { unreachable!() };
/// assert_eq!(balance.account.as_str(), "Assets:Bank:Checking");
/// assert_eq!(balance.amount.value, 10.0);
/// assert_eq!(balance.amount.currency.as_str(), "CHF");
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Balance<D> {
    /// Account being asserted
    pub account: Account,
    /// Amount the amount should have on the date
    pub amount: Amount<D>,
}

/// Pad directive
///
/// # Example
/// ```
/// # use beancount_parser_2::DirectiveContent;
/// let raw = "2014-06-01 pad Assets:BofA:Checking Equity:Opening-Balances";
/// let file = beancount_parser_2::parse::<f64>(raw).unwrap();
/// let DirectiveContent::Pad(pad) = &file.directives[0].content else { unreachable!() };
/// assert_eq!(pad.account.as_str(), "Assets:BofA:Checking");
/// assert_eq!(pad.source_account.as_str(), "Equity:Opening-Balances");
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Pad {
    /// Account being padded
    pub account: Account,
    /// Source account from which take the money
    pub source_account: Account,
}

pub(super) fn parse(input: Span<'_>) -> IResult<'_, Account> {
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
                satisfy(|c: char| c.is_uppercase() || c.is_ascii_digit()),
                take_while(|c: char| c.is_alphanumeric() || c == '-'),
            ),
        ))),
    ))(input)?;
    Ok((input, Account(Arc::from(*name.fragment()))))
}

pub(super) fn open(input: Span<'_>) -> IResult<'_, Open> {
    let (input, account) = parse(input)?;
    let (input, _) = space0(input)?;
    let (input, currencies) = opt(currencies)(input)?;
    Ok((
        input,
        Open {
            account,
            currencies: currencies.unwrap_or_default(),
        },
    ))
}

fn currencies(input: Span<'_>) -> IResult<'_, HashSet<Currency>> {
    let (input, first) = amount::currency(input)?;
    let sep = delimited(space0, char(','), space0);
    let mut iter = iterator(input, preceded(sep, amount::currency));
    let mut currencies = HashSet::new();
    currencies.insert(first);
    currencies.extend(iter.into_iter());
    let (input, _) = iter.finish()?;
    Ok((input, currencies))
}

pub(super) fn close(input: Span<'_>) -> IResult<'_, Close> {
    let (input, account) = parse(input)?;
    Ok((input, Close { account }))
}

pub(super) fn balance<D: Decimal>(input: Span<'_>) -> IResult<'_, Balance<D>> {
    let (input, account) = parse(input)?;
    let (input, _) = space1(input)?;
    let (input, amount) = amount::parse(input)?;
    Ok((input, Balance { account, amount }))
}

pub(super) fn pad(input: Span<'_>) -> IResult<'_, Pad> {
    let (input, account) = parse(input)?;
    let (input, _) = space1(input)?;
    let (input, source_account) = parse(input)?;
    Ok((
        input,
        Pad {
            account,
            source_account,
        },
    ))
}
