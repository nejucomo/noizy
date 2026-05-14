use eframe::egui::{Context, Key::Escape, ViewportCommand::Close};

#[derive(Default)]
pub(crate) struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        use eframe::egui::CentralPanel;

        CentralPanel::default().show(ctx, |ui| {
            if ui.input(|i| i.key_pressed(Escape)) {
                ctx.send_viewport_cmd(Close);
            }

            ui.label("Hello World!");
        });
    }
}
