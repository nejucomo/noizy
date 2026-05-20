use derive_new::new;
use eframe::egui::{Area, Frame, Id, InnerResponse, Pos2, Rect, Sense, TextStyle, Ui, Vec2};

use crate::consts::GOLDEN_RATIO;
use crate::dragpatch::DragPatch;
use crate::l2g::LerpToGamma;
use crate::widgable::{UiWidgableExt as _, Widgable};

#[derive(new)]
#[new(visibility = "pub(crate)")]
pub(crate) struct WidgetBox<T> {
    pub(crate) id: Id,
    pub(crate) pos: Pos2,
    l2g: LerpToGamma,
    pub(crate) inner: T,
}

impl<T> Widgable for WidgetBox<T>
where
    T: Widgable,
{
    type Inner = T::Inner;

    fn widge_into(&mut self, ui: &mut Ui) -> InnerResponse<Self::Inner> {
        Area::new(self.id)
            .current_pos(self.pos)
            .show(ui.ctx(), |ui| {
                let mut f = Frame::window(&ui.ctx().style());

                self.l2g.mix_into(&mut f.fill);

                f.show(ui, |ui| {
                    ui.horizontal_centered(|ui| {
                        let width = ui.text_style_height(&TextStyle::Body) / GOLDEN_RATIO;

                        // Reserve horizontal space, but don't prescribe row height.
                        let (reserved_rect, _) =
                            ui.allocate_exact_size(Vec2::new(width, 0.0), Sense::drag());

                        let inner_resp = ui.widge(&mut self.inner);
                        let inner_rect = inner_resp.response.rect;

                        let drag_rect = Rect::from_min_size(
                            Pos2::new(reserved_rect.min.x, inner_rect.min.y),
                            Vec2::new(width, inner_rect.height()),
                        );

                        ui.widge(&mut DragPatch::new(&mut self.pos, drag_rect));
                        inner_resp.inner
                    })
                    .inner
                })
                .inner
            })
    }
}
