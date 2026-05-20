use derive_new::new;
use eframe::egui::{Area, Frame, Id, InnerResponse, Pos2, Ui};

use crate::draghandle::DragHandle;
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
                    ui.horizontal(|ui| {
                        ui.widge(&mut DragHandle::new(&mut self.pos));
                        ui.widge(&mut self.inner).inner
                    })
                    .inner
                })
                .inner
            })
    }
}
