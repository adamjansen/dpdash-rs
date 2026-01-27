use gpui::{prelude::*, *};
use gpui_component::{
    ActiveTheme as _, Icon, IconName, h_flex,
    input::{Input, InputEvent, InputState},
    resizable::{h_resizable, resizable_panel},
    sidebar::{Sidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem},
    v_flex,
};
use gpui_component_assets::Assets;

use dpdash::workspace;

pub struct DpDashApp {}

impl DpDashApp {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for DpDashApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        dpdash::init(cx);

        workspace::open_new(cx, |_, _, _| {}).detach();
    });
}
