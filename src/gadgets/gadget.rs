use eframe::egui::{Response, Ui, Widget};

use self::Gadget::*;

pub(super) enum Gadget {
    Sin,
}

impl Widget for &mut Gadget {
    fn ui(self, _ui: &mut Ui) -> Response {
        match self {
            Sin => todo!("`Widget::ui` for `Gadget::Sin`"),
        }
    }
}
