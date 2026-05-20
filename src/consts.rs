use eframe::egui::Color32;

use crate::l2g::LerpToGamma;

pub(crate) const PENDING_L2G: LerpToGamma = LerpToGamma {
    to: Color32::LIGHT_YELLOW,
    gamma: 0.6,
};

pub(crate) const GADGET_L2G: LerpToGamma = LerpToGamma {
    to: Color32::PURPLE,
    gamma: 0.2,
};
