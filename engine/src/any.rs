mod inner;

use std::str::FromStr;

use crate::Gadget;
use crate::any::inner::ParseError;

pub struct AnyGadget(self::inner::Inner);

impl FromStr for AnyGadget {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(AnyGadget)
    }
}

impl Gadget for AnyGadget {
    fn initializer(&self) -> &str {
        self.0.initializer()
    }
}
