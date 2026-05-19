use eframe::egui::{Response, Ui, Widget};

use crate::gadgets::sin::Sin;

pub(crate) struct Gadget(Inner);

enum Inner {
    Sin(Sin),
}

impl Gadget {
    pub(crate) fn parse_initializer_opt(init: &str) -> Option<Self> {
        if init == "sin" {
            Some(Gadget(Inner::Sin(Sin::default())))
        } else {
            None
        }
    }
}

impl Widget for &mut Gadget {
    fn ui(self, ui: &mut Ui) -> Response {
        use Inner::*;

        match &self.0 {
            Sin(s) => s.ui(ui),
        }
    }
}
