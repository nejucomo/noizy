use eframe::egui::{CentralPanel, Modifiers, Sense, Vec2};
use eframe::egui::{Context, Key::Escape, ViewportCommand::Close};

use crate::consts::{GADGET_L2G, PENDING_L2G};
use crate::gadgets::Gadget;
use crate::idgen::IdGen;
use crate::iwidget::UiExt as _;
use crate::pending::Pending;
use crate::widgetbox::WidgetBox;

#[derive(Default)]
pub(crate) struct App {
    idgen: IdGen,
    gadgets: Vec<WidgetBox<Gadget>>,
    pending: Option<WidgetBox<Pending>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.input_mut(|i| i.consume_key(Modifiers::COMMAND, Escape)) {
                ctx.send_viewport_cmd(Close);
            }

            let (_, resp) = ui.allocate_exact_size(
                Vec2::new(ui.available_width(), ui.available_height()),
                Sense::click(),
            );

            if resp.clicked() {
                let id = self.idgen.next_id();
                let pos = resp.interact_pointer_pos().unwrap();
                // Overwrite any other pending, if it exists:
                self.pending = Some(WidgetBox::new(id, pos, PENDING_L2G, Pending::default()));
            }

            for gbox in self.gadgets.iter_mut() {
                ui.add_iresp(gbox);
            }

            if let Some(mut pending) = self.pending.take() {
                let iresp = ui.add_iresp(&mut pending);
                if let Some(gadget) = iresp.inner {
                    self.gadgets.push(WidgetBox::new(
                        self.idgen.next_id(),
                        pending.get_pos(ctx),
                        GADGET_L2G,
                        gadget,
                    ));
                } else {
                    // Put back pending:
                    self.pending = Some(pending);
                }
            }

            resp
        });
    }
}
