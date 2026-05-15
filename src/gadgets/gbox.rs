//! # Design Notes
//!
//! Some gnarly state normalization in here in order to (a) support [Pending] gadget initializer widgets which replace themselves with a [Gadget] on <return> (if valid) and (b) prevent incoherent runtime states. [Pending] replacement here also enables [Gadget] rendering to be simple/clean.
use eframe::egui::{Area, Color32, Frame, Id, Pos2, Response, Ui, Widget};

use crate::consts::{
    GADGET_FILL, GADGET_FILL_GAMMA, GADGET_FILL_PENDING, GADGET_FILL_PENDING_GAMMA,
};
use crate::gadgets::gadget::Gadget;
use crate::gadgets::pending::Pending;

pub(crate) struct GadgetBox {
    id: Id,
    inner: InnerState,
}

enum InnerState {
    FirstFrame(Pos2),
    Widget(InnerWidget),
}

enum InnerWidget {
    Pending(Pending),
    Gadget(Gadget),
}

impl GadgetBox {
    pub(crate) fn new(id: Id, pos: Pos2) -> Self {
        GadgetBox {
            id,
            inner: InnerState::FirstFrame(pos),
        }
    }
}

impl Widget for &mut GadgetBox {
    fn ui(self, ui: &mut Ui) -> Response {
        let (area, iw) = self.inner.area_and_widget(self.id);

        area.show(ui.ctx(), |ui| {
            let style = ui.ctx().style();
            let (mixin, gamma) = iw.fill_color_and_gamma();
            Frame::window(&style)
                .fill(style.visuals.window_fill.lerp_to_gamma(mixin, gamma))
                .show(ui, |ui| ui.add(iw));
        })
        .response
    }
}

impl InnerState {
    fn area_and_widget(&mut self, id: Id) -> (Area, &mut InnerWidget) {
        use InnerState::*;

        let mut area = Area::new(id);

        if let FirstFrame(pos) = *self {
            *self = Widget(InnerWidget::Pending(Pending::default()));
            area = area.default_pos(pos);
        }

        match self {
            Widget(innermut) => (area, innermut),
            FirstFrame(p) => unreachable!("{p:?}"),
        }
    }
}

impl InnerWidget {
    fn fill_color_and_gamma(&self) -> (Color32, f32) {
        use InnerWidget::*;
        match self {
            Pending(_) => (GADGET_FILL_PENDING, GADGET_FILL_PENDING_GAMMA),
            Gadget(_) => (GADGET_FILL, GADGET_FILL_GAMMA),
        }
    }
}

impl Widget for &mut InnerWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        use InnerWidget::*;

        match self {
            Pending(pending) => {
                let mut iresp = pending.show(ui);
                if let Some(gadget) = iresp.inner.take() {
                    *self = Gadget(gadget);
                }
                iresp.response
            }

            Gadget(gadget) => ui.add(&*gadget),
        }
    }
}
