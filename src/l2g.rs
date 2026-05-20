use eframe::egui::Color32;

pub(crate) struct LerpToGamma {
    pub(crate) to: Color32,
    pub(crate) gamma: f32,
}

impl LerpToGamma {
    pub(crate) fn mix_into(&self, target: &mut Color32) {
        *target = self.apply_to(*target);
    }

    pub(crate) fn apply_to(&self, source: Color32) -> Color32 {
        source.lerp_to_gamma(self.to, self.gamma)
    }

    pub(crate) fn scale_gamma(mut self, factor: f32) -> Self {
        self.gamma *= factor;
        self
    }
}
