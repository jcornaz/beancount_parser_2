use nom::character::complete::space1;

use crate::{string, IResult, Span};

/// An event
///
/// # Example
/// ```
/// # use beancount_parser_2::DirectiveContent;
/// let input = r#"2023-05-31 event "Location" "Switzerland""#;
/// let beancount = beancount_parser_2::parse::<&str, f64>(input).unwrap();
/// let DirectiveContent::Event(ref event) = beancount.directives[0].content else { unreachable!() };
/// assert_eq!(event.name, "Location");
/// assert_eq!(event.value, "Switzerland");
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Event<S> {
    /// Name
    pub name: S,
    /// Value
    pub value: S,
}

pub(super) fn parse<'a, S: From<&'a str>>(input: Span<'a>) -> IResult<'a, Event<S>> {
    let (input, name) = string(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = string(input)?;
    Ok((input, Event { name, value }))
}
