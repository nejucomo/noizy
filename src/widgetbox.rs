use eframe::egui::{Area, AreaState, Context, Frame, Id, InnerResponse, Pos2, Ui};

use crate::iwidget::IWidget;
use crate::l2g::LerpToGamma;

pub(crate) struct WidgetBox<T> {
    id: Id,
    initpos: Option<Pos2>,
    l2g: LerpToGamma,
    pub(crate) inner: T,
}

impl<T> WidgetBox<T> {
    pub(crate) fn new(id: Id, initpos: Pos2, l2g: LerpToGamma, inner: T) -> Self {
        let initpos = Some(initpos);
        Self {
            id,
            initpos,
            l2g,
            inner,
        }
    }

    pub(crate) fn get_pos(&self, ctx: &Context) -> Pos2 {
        AreaState::load(ctx, self.id).unwrap().left_top_pos()
    }
}

impl<'a, T> IWidget for &'a mut WidgetBox<T>
where
    &'a mut T: IWidget,
{
    type Inner = <&'a mut T as IWidget>::Inner;

    fn ui_iresp(self, ui: &mut Ui) -> InnerResponse<Self::Inner> {
        let mut area = Area::new(self.id);

        if let Some(p) = self.initpos.take() {
            // todo: See if we can call this unconditionally and drop the Option/switch:
            area = area.default_pos(p);
        }

        area.show(ui.ctx(), |ui| {
            let mut f = Frame::window(&ui.ctx().style());
            self.l2g.mix_into(&mut f.fill);

            f.show(ui, |ui| (&mut self.inner).ui_iresp(ui).inner).inner
        })
    }
}
