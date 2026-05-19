use eframe::egui::{InnerResponse, Ui};

use crate::gadgets::Gadget;
use crate::iwidget::IWidget;

#[derive(Default)]
pub(super) struct Pending {
    initializer: String,
    request_initial_focus: bool,
    was_focused: bool,
}

impl IWidget for &mut Pending {
    type Inner = Option<Gadget>;

    fn ui_iresp(self, ui: &mut Ui) -> InnerResponse<Self::Inner> {
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
