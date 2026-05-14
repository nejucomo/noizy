use eframe::egui::Id;

#[derive(Default)]
pub(crate) struct IdGen(u64);

impl IdGen {
    pub(crate) fn next_id(&mut self) -> Id {
        self.0 += 1;
        Id::new(self.0)
    }
}
