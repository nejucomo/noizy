use eframe::egui::{InnerResponse, Ui, Widget};

/// Generalize [Widget] to provide impl-specific [InnerResponse]s
pub(crate) trait IWidget {
    type Inner;

    fn ui_iresp(self, ui: &mut Ui) -> InnerResponse<Self::Inner>;
}

impl<B> IWidget for B
where
    B: Widget,
{
    type Inner = ();

    fn ui_iresp(self, ui: &mut Ui) -> InnerResponse<Self::Inner> {
        InnerResponse::new((), ui.add(self))
    }
}

pub(crate) trait UiExt {
    fn add_iresp<W: IWidget>(&mut self, widget: W) -> InnerResponse<W::Inner>;
}

impl UiExt for Ui {
    fn add_iresp<W: IWidget>(&mut self, widget: W) -> InnerResponse<W::Inner> {
        widget.ui_iresp(self)
    }
}
