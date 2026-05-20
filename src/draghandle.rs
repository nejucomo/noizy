use derive_new::new;
use eframe::egui::{InnerResponse, Pos2, Sense, Stroke, TextStyle, Ui, Vec2};

use crate::consts::GOLDEN_RATIO;
use crate::widgable::Widgable;

#[derive(new)]
#[new(visibility = "pub(crate)")]
pub struct DragHandle<'a> {
    pos: &'a mut Pos2,
}

impl<'a> Widgable for DragHandle<'a> {
    type Inner = ();

    fn widge_into(&mut self, ui: &mut Ui) -> InnerResponse<Self::Inner> {
        let height = ui.text_style_height(&TextStyle::Body);
        let width = height / GOLDEN_RATIO;
        let size = Vec2::new(width, height);

        let (rect, resp) = ui.allocate_exact_size(size, Sense::drag());

        // // Diagnostic:
        // ui.painter()
        //     .rect_filled(rect, 0, eframe::egui::Color32::BLACK);

        let dot_radius = (height * 0.08).max(1.0);
        let color = ui.visuals().widgets.inactive.fg_stroke.color;
        let stroke = Stroke::new(dot_radius * 0.5, color);

        let cx = rect.center().x;

        // A vertical "grip": three dots, centered in a body-text-height square.
        for y_frac in [0.30, 0.50, 0.70] {
            let center = Pos2::new(cx, rect.top() + height * y_frac);
            ui.painter().circle_stroke(center, dot_radius, stroke);
        }

        if resp.dragged() {
            *self.pos += resp.drag_delta();
        }

        InnerResponse::new((), resp)
    }
}
