use eframe::egui::{CentralPanel, Modifiers, Rect, Scene, Sense, Vec2};
use eframe::egui::{Context, Key::Escape, ViewportCommand::Close};

use crate::gadget::GadgetBox;
use crate::idgen::IdGen;

pub(crate) struct App {
    idgen: IdGen,
    scenerect: Rect,
    gadgets: Vec<GadgetBox>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            idgen: IdGen::default(),
            scenerect: Rect::ZERO,
            gadgets: vec![],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.input_mut(|i| i.consume_key(Modifiers::COMMAND, Escape)) {
                ctx.send_viewport_cmd(Close);
            }

            let resp = Scene::new()
                .sense(Sense::hover())
                .show(ui, &mut self.scenerect, |ui| {
                    let mut r = ui.allocate_response(Vec2::ZERO, Sense::hover());
                    for gbox in self.gadgets.iter_mut() {
                        r |= ui.add(gbox);
                    }
                    r
                })
                .inner;

            if resp.clicked() {
                let id = self.idgen.next_id();
                let pos = resp.interact_pointer_pos().unwrap();
                todo!("{id:?} {pos:?}");
            }
        });
    }
}
