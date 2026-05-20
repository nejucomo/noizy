use std::str::FromStr;

use thiserror::Error;

use crate::sin::Sin;

pub struct AnyGadget(Inner);

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unknown gadget: {0:?}")]
    UnknownGadget(String),
}

impl FromStr for AnyGadget {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(AnyGadget)
    }
}

enum Inner {
    Sin(Sin),
}

impl FromStr for Inner {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s == "sin" {
            Ok(Inner::Sin(Sin::default()))
        } else {
            Err(ParseError::UnknownGadget(s.to_string()))
        }
    }
}
