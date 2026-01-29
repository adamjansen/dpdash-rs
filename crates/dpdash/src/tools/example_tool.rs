use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, Render, Styled as _, Window, px,
};

use gpui_component::{dock::PanelControl, text::markdown};

use crate::Tool;

pub struct ExampleTool {
    focus_handle: FocusHandle,
}

impl ExampleTool {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Tool for ExampleTool {
    fn title() -> &'static str {
        "Welcome"
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

impl Focusable for ExampleTool {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ExampleTool {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        markdown("## Welcome!\n\nThis is an **example**.")
            .px_4()
            .scrollable(true)
            .selectable(true)
    }
}
