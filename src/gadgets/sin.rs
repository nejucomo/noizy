use eframe::egui::{Response, Ui, Widget};

#[derive(Default)]
pub(super) struct Sin {}

impl Widget for &Sin {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label("FIXME: placeholder Sin Widget")
    }
}
