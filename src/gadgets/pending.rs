use eframe::egui::{InnerResponse, TextEdit, Ui};

use crate::gadgets::gadget::Gadget;
use crate::tuning::COLOR_TEXT_INVALID_INITIALIZER;

#[derive(Default)]
pub(super) struct Pending {
    initializer: String,
    was_focused: bool,
}

impl Pending {
    pub(super) fn show(&mut self, ui: &mut Ui) -> InnerResponse<Option<Gadget>> {
        let text = TextEdit::singleline(&mut self.initializer).frame(false);

        let text = if self.was_focused {
            text
        } else {
            text.text_color(COLOR_TEXT_INVALID_INITIALIZER)
        };

        let resp = ui.add(text);

        let optg = if resp.lost_focus() {
            self.parse_initializer()
        } else {
            None
        };

        self.was_focused = resp.has_focus();

        InnerResponse::new(optg, resp)
    }

    fn parse_initializer(&self) -> Option<Gadget> {
        if self.initializer == "sin" {
            Some(Gadget::Sin)
        } else {
            None
        }
    }
}
