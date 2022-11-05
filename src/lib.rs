mod lexer;
mod parser;

use std::fmt;
use std::str::FromStr;

use parser::{ParseNsidError, Parser};

/// A NameSpaced ID (NSID).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Nsid {
    authority_segments: Vec<String>,
    name: String,
}

impl Nsid {
    pub fn authority(&self) -> String {
        self.authority_segments
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>()
            .join(".")
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Nsid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.authority_segments.join("."), self.name)
    }
}

impl FromStr for Nsid {
    type Err = ParseNsidError;

    fn from_str(nsid: &str) -> Result<Self, Self::Err> {
        let parser = Parser::new(nsid);

        let segments = parser.parse()?;

        match segments.split_last() {
            Some((name, authority)) => {
                if authority.len() < 2 {
                    return Err(ParseNsidError::TooFewSegments);
                }

                Ok(Self {
                    authority_segments: authority.iter().map(ToString::to_string).collect(),
                    name: name.to_string(),
                })
            }
            None => Err(ParseNsidError::TooFewSegments),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_valid_nsids() {
        let nsid = Nsid::from_str("com.example.foo").unwrap();

        assert_eq!(nsid.authority(), "example.com");
        assert_eq!(nsid.name(), "foo");
        assert_eq!(format!("{}", nsid), "com.example.foo");

        let nsid = Nsid::from_str("com.example.*").unwrap();

        assert_eq!(nsid.authority(), "example.com");
        assert_eq!(nsid.name(), "*");
        assert_eq!(format!("{}", nsid), "com.example.*");

        let nsid = Nsid::from_str("com.long-thing1.cool.fooBarBaz").unwrap();

        assert_eq!(nsid.authority(), "cool.long-thing1.com");
        assert_eq!(nsid.name(), "fooBarBaz");
        assert_eq!(format!("{}", nsid), "com.long-thing1.cool.fooBarBaz");
    }

    #[test]
    fn it_does_not_parse_invalid_nsids() {
        assert_eq!(
            Nsid::from_str("com.1example.foo"),
            Err(ParseNsidError::SyntaxError("1".to_string()))
        );
        assert_eq!(
            Nsid::from_str("com.example!.foo"),
            Err(ParseNsidError::SyntaxError("!".to_string()))
        );
        assert_eq!(
            Nsid::from_str("com.example.*.foo"),
            Err(ParseNsidError::SyntaxError("*".to_string()))
        );
        assert_eq!(Nsid::from_str("foo"), Err(ParseNsidError::TooFewSegments));
        assert_eq!(
            Nsid::from_str("foo/bar"),
            Err(ParseNsidError::SyntaxError("/".to_string()))
        );
    }
}
