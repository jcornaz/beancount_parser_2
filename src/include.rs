use std::path::{Path, PathBuf};

use nom::{
    bytes::complete::tag,
    character::complete::space1,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

use crate::string::string;

/// Include directive
#[derive(Clone, Debug)]
pub struct Include {
    path: PathBuf,
}

impl Include {
    /// Path to include
    #[must_use]
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

pub(crate) fn include(input: &str) -> IResult<&str, Include> {
    map(preceded(tuple((tag("include"), space1)), string), |path| {
        Include { path: path.into() }
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::combinator::all_consuming;

    #[test]
    fn valid_include_directive() {
        let (_, inc) = include(r#"include "abc.beancount""#).unwrap();
        assert_eq!(inc.path().to_str(), Some("abc.beancount"));
    }

    #[rstest]
    fn invalid(#[values("include", r#"include "a" "b""#)] input: &str) {
        assert!(matches!(
            all_consuming(include)(input),
            Err(nom::Err::Error(_))
        ));
    }
}
