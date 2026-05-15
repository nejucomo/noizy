use eframe::egui::{Response, Ui, Widget};

use crate::gadgets::sin::Sin;

pub(super) enum Gadget {
    Sin(Sin),
}

impl Gadget {
    pub(super) fn parse_initializer_opt(init: &str) -> Option<Self> {
        if init == "sin" {
            Some(Gadget::Sin(Sin::default()))
        } else {
            None
        }
    }
}

impl Widget for &Gadget {
    fn ui(self, ui: &mut Ui) -> Response {
        use self::Gadget::*;

        match self {
            Sin(s) => s.ui(ui),
        }
    }
}
