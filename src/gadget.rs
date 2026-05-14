use derive_new::new;
use eframe::egui::{Area, Frame, Id, Pos2, Response, Ui, Widget};

use self::Gadget::*;

#[derive(new)]
pub(crate) struct GadgetBox {
    id: Id,
    #[new(into)]
    newpos: Option<Pos2>,
    #[new(default)]
    gadget: Gadget,
}

impl Widget for &mut GadgetBox {
    fn ui(self, ui: &mut Ui) -> Response {
        let area = Area::new(self.id);

        let area = if let Some(pos) = self.newpos.take() {
            dbg!(pos, ui.min_rect(), ui.max_rect());
            area.default_pos(pos)
        } else {
            area
        };

        area.show(ui.ctx(), |ui| {
            Frame::window(&ui.ctx().style()).show(ui, |ui| ui.add(&mut self.gadget));
        })
        .response
    }
}

enum Gadget {
    /// A "non-instantiated" pending initializer string
    Pending(String),
    #[allow(dead_code)]
    Sin,
}

impl Default for Gadget {
    fn default() -> Self {
        Pending("".to_string())
    }
}

impl Widget for &mut Gadget {
    fn ui(self, ui: &mut Ui) -> Response {
        match self {
            Pending(cmd) => {
                let resp = ui.text_edit_singleline(cmd);
                if resp.lost_focus() {
                    todo!("{cmd:?}");
                }
                resp
            }
            Sin => todo!("`Widget::ui` for `Gadget::Sin`"),
        }
    }
}
