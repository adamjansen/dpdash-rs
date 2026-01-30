use std::rc::Rc;

use gpui::{
    AnyElement, App, Context, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    div, px, rems,
};

use gpui_component::{ActiveTheme, IconName, divider::Divider};

use crate::icons::DpIconName;

type ChildFunc = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;

pub struct StatusBar {
    child: ChildFunc,
    _subscriptions: Vec<Subscription>,
}

impl StatusBar {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            child: Rc::new(|_, _| div().into_any_element()),
            _subscriptions: vec![],
        }
    }

    pub fn child<F, E>(mut self, f: F) -> Self
    where
        E: IntoElement,
        F: Fn(&mut Window, &mut App) -> E + 'static,
    {
        self.child = Rc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .h(rems(1.8))
            .w_full()
            .bg(theme.sidebar)
            .border_t(px(1.0))
            .border_color(theme.border)
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(rems(0.8))
                    .child(
                        div()
                            .px(rems(0.4))
                            .py(rems(0.1))
                            .rounded(theme.radius)
                            .text_xs()
                            .text_color(theme.accent_foreground)
                            .child(IconName::Network),
                    )
                    .child(Divider::vertical()),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(rems(0.8))
                    .child(Divider::vertical())
                    .child(
                        div()
                            .px(rems(0.4))
                            .py(rems(0.1))
                            .rounded(theme.radius)
                            .text_xs()
                            .text_color(theme.accent_foreground)
                            .child(DpIconName::Bug),
                    ),
            )
    }
}
