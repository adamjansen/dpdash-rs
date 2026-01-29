use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, Render, Styled as _, Window, px,
};

use gpui_component::{dock::PanelControl, text::markdown};

use crate::Tool;

pub struct DeviceManagerTool {
    focus_handle: FocusHandle,
}

impl DeviceManagerTool {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Tool for DeviceManagerTool {
    fn title() -> &'static str {
        "Device Manager"
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

impl Focusable for DeviceManagerTool {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DeviceManagerTool {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        markdown("# Device Manager!\n\n- Device 1\n- Device 2\n- Device 3.")
            .px_4()
            .scrollable(true)
            .selectable(true)
    }
}
