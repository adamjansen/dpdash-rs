use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, ParentElement, Render, Styled as _,
    Window, div, px,
};

use gpui_component::{
    description_list::{DescriptionItem, DescriptionList},
    dock::PanelControl,
};

use crate::Tool;

pub struct PropertyEditorTool {
    focus_handle: FocusHandle,
}

impl PropertyEditorTool {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Tool for PropertyEditorTool {
    fn title() -> &'static str {
        "Property Editor"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn zoomable() -> Option<PanelControl> {
        None
    }

    fn paddings() -> gpui::Pixels {
        px(0.)
    }
}

impl Focusable for PropertyEditorTool {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PropertyEditorTool {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex()
            .flex_col()
            .items_start()
            .justify_between()
            .child(
                DescriptionList::horizontal()
                    .columns(1)
                    .label_width(px(200.0))
                    .children([
                        DescriptionItem::new("Version").value("0.1.0").span(1),
                        DescriptionItem::new("Status").value("Active").span(1),

                        DescriptionItem::Divider, // Full-width divider

                        DescriptionItem::new("Description").value(
                            "A comprehensive UI component library for building desktop applications with GPUI"
                        ),

                    ]) )
    }
}
