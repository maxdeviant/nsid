use std::fmt;
use std::str::FromStr;

// Grammar:
//
// alpha     = "a" / "b" / "c" / "d" / "e" / "f" / "g" / "h" / "i" / "j" / "k" / "l" / "m" / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z" / "A" / "B" / "C" / "D" / "E" / "F" / "G" / "H" / "I" / "J" / "K" / "L" / "M" / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z"
// number    = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9" / "0"
// delim     = "."
// segment   = alpha *( alpha / number / "-" )
// authority = segment *( delim segment )
// name      = segment
// nsid      = authority delim name
// nsid-ns   = authority delim "*"

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

#[derive(Debug)]
pub enum ParseNsidError {
    TooFewSegments,
}

impl FromStr for Nsid {
    type Err = ParseNsidError;

    fn from_str(nsid: &str) -> Result<Self, Self::Err> {
        match nsid.split(".").collect::<Vec<_>>().split_last() {
            Some((name, authority)) => Ok(Self {
                authority_segments: authority.iter().map(ToString::to_string).collect(),
                name: name.to_string(),
            }),
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
}
