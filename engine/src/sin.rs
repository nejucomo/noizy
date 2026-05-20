use crate::Gadget;

#[derive(Default)]
pub(crate) struct Sin {}

impl Gadget for Sin {
    fn initializer(&self) -> &str {
        "sin"
    }
}
