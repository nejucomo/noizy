use eframe::egui::{InnerResponse, Ui};
use noizy_engine::AnyGadget;

use crate::consts::PENDING_L2G;
use crate::widgable::Widgable;

#[derive(Default)]
pub(super) struct Pending {
    initializer: String,
    request_initial_focus: bool,
    was_focused: bool,
}

impl Widgable for Pending {
    type Inner = Option<AnyGadget>;

    fn widge_into(&mut self, ui: &mut Ui) -> InnerResponse<Self::Inner> {
        let vis = ui.visuals_mut();
        vis.text_edit_bg_color = Some(
            PENDING_L2G
                .scale_gamma(0.2)
                .apply_to(vis.text_edit_bg_color()),
        );

        let resp = ui.text_edit_singleline(&mut self.initializer);

        if !self.request_initial_focus {
            resp.request_focus();
            self.request_initial_focus = true;
        }

        let optg = if resp.lost_focus() {
            // TODO: Display any parse error as a tooltip-style indicator
            self.initializer.parse().ok()
        } else {
            None
        };

        self.was_focused = resp.has_focus();

        InnerResponse::new(optg, resp)
    }
}
