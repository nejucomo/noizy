use eframe::egui::{CentralPanel, Modifiers, Sense, Vec2};
use eframe::egui::{Context, Key::Escape, ViewportCommand::Close};

use crate::gadgets::GadgetBox;
use crate::idgen::IdGen;

#[derive(Default)]
pub(crate) struct App {
    idgen: IdGen,
    gadgets: Vec<GadgetBox>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.input_mut(|i| i.consume_key(Modifiers::COMMAND, Escape)) {
                ctx.send_viewport_cmd(Close);
            }

            for gbox in self.gadgets.iter_mut() {
                ui.add(gbox);
            }

            let (_, resp) = ui.allocate_exact_size(
                Vec2::new(ui.available_width(), ui.available_height()),
                Sense::click(),
            );

            if resp.clicked() {
                let id = self.idgen.next_id();
                let pos = resp.interact_pointer_pos().unwrap();
                self.gadgets.push(GadgetBox::new(id, pos));
            }

            resp
        });
    }
}
