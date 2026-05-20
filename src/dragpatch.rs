use derive_new::new;
use eframe::egui::{InnerResponse, Pos2, Rect, Sense, TextStyle, Ui};

use crate::consts::GOLDEN_RATIO;
use crate::consts::drag_patch::{DOT_GAMMA, DOTS_PER_TEXT_HEIGHT};
use crate::widgable::Widgable;

#[derive(new)]
#[new(visibility = "pub(crate)")]
pub struct DragPatch<'a> {
    pos: &'a mut Pos2,
    rect: Rect,
}

impl<'a> Widgable for DragPatch<'a> {
    type Inner = ();

    fn widge_into(&mut self, ui: &mut Ui) -> InnerResponse<Self::Inner> {
        // Dot radius scaled to text size:
        let radius = ui.text_style_height(&TextStyle::Body) / DOTS_PER_TEXT_HEIGHT;

        let resp = ui.allocate_rect(self.rect, Sense::drag());

        let painter = ui.painter();
        let color = ui.visuals().text_color().gamma_multiply(DOT_GAMMA);
        let step_delta = 2.0 * radius * GOLDEN_RATIO;

        let steps_x = (self.rect.width() / step_delta).floor() as usize + 1;
        let steps_y = (self.rect.height() / step_delta).floor() as usize + 1;

        let cleft = self.rect.left() + radius;
        let ctop = self.rect.top() + radius;

        for x_step in 0..steps_x {
            for y_step in 0..steps_y {
                let x = cleft + x_step as f32 * step_delta;
                let y = ctop + y_step as f32 * step_delta;
                painter.circle_filled(Pos2 { x, y }, radius, color);
            }
        }

        if resp.dragged() {
            *self.pos += resp.drag_delta();
        }

        InnerResponse::new((), resp)
    }
}
