use std::str::FromStr;

use thiserror::Error;

use crate::Gadget;
use crate::sin::Sin;

use self::Inner::*;

pub(super) enum Inner {
    ISin(Sin),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unknown gadget: {0:?}")]
    UnknownGadget(String),
}

impl FromStr for Inner {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s == "sin" {
            Ok(ISin(Sin::default()))
        } else {
            Err(ParseError::UnknownGadget(s.to_string()))
        }
    }
}

impl Gadget for Inner {
    fn initializer(&self) -> &str {
        match self {
            ISin(sin) => sin.initializer(),
        }
    }
}
