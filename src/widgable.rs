use eframe::egui::{InnerResponse, Ui};
use noizy_engine::{AnyGadget, Gadget as _};

/// Generalize [Widget] to provide impl-specific [InnerResponse]s
pub(crate) trait Widgable {
    type Inner;

    fn widge_into(&mut self, ui: &mut Ui) -> InnerResponse<Self::Inner>;
}

// TODO: Use `#[extension(...)]` from `extension-traits`
pub(crate) trait UiWidgableExt {
    fn widge<W: Widgable>(&mut self, widget: &mut W) -> InnerResponse<W::Inner>;
}

impl UiWidgableExt for Ui {
    fn widge<W: Widgable>(&mut self, widget: &mut W) -> InnerResponse<W::Inner> {
        widget.widge_into(self)
    }
}

impl Widgable for AnyGadget {
    type Inner = ();

    fn widge_into(&mut self, ui: &mut Ui) -> InnerResponse<Self::Inner> {
        InnerResponse::new((), ui.label(self.initializer()))
    }
}
