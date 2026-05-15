use eframe::egui::{InnerResponse, Ui};

use crate::gadgets::gadget::Gadget;

#[derive(Default)]
pub(super) struct Pending {
    initializer: String,
    request_initial_focus: bool,
    was_focused: bool,
}

impl Pending {
    pub(super) fn show(&mut self, ui: &mut Ui) -> InnerResponse<Option<Gadget>> {
        let resp = ui.text_edit_singleline(&mut self.initializer);

        if !self.request_initial_focus {
            resp.request_focus();
            self.request_initial_focus = true;
        }

        let optg = if resp.lost_focus() {
            Gadget::parse_initializer_opt(&self.initializer)
        } else {
            None
        };

        self.was_focused = resp.has_focus();

        InnerResponse::new(optg, resp)
    }
}
