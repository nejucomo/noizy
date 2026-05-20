use eframe::egui::Color32;

use crate::l2g::LerpToGamma;

pub(crate) const GOLDEN_RATIO: f32 = 1.618_034; // φ = (1 + √5) / 2 

pub(crate) const PENDING_L2G: LerpToGamma = LerpToGamma {
    to: Color32::LIGHT_YELLOW,
    gamma: 0.12,
};

pub(crate) const GADGET_L2G: LerpToGamma = LerpToGamma {
    to: Color32::PURPLE,
    gamma: 0.2,
};
