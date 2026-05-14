use derive_new::new;
use eframe::egui::{Area, Id, Response, Ui, Widget};

use self::Gadget::*;

#[derive(new)]
pub(crate) struct GadgetBox {
    id: Id,
    #[new(default)]
    gadget: Gadget,
}

impl Widget for &mut GadgetBox {
    fn ui(self, ui: &mut Ui) -> Response {
        Area::new(self.id)
            .show(ui.ctx(), |ui| ui.add(&mut self.gadget))
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
