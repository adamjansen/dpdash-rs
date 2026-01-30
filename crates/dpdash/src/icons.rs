use gpui::{App, IntoElement, RenderOnce, SharedString, Window};
use gpui_component::{Icon, IconNamed};

#[allow(dead_code)]
#[derive(IntoElement, Clone)]
pub enum DpIconName {
    Binary,
    Bug,
    BugOff,
}

impl IconNamed for DpIconName {
    fn path(self) -> SharedString {
        match self {
            Self::Binary => "icons/binary.svg",
            Self::Bug => "icons/bug.svg",
            Self::BugOff => "icons/bug-off.svg",
        }
        .into()
    }
}

impl RenderOnce for DpIconName {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        Icon::new(self).into_any_element()
    }
}
