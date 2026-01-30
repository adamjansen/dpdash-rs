use std::rc::Rc;

use gpui::*;
use gpui_component::{ActiveTheme, IconName, button::*, divider::Divider, label::Label, tag::Tag};

use crate::icons::DpIconName;

pub struct StatusBar {
    child: Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>,
    _subscriptions: Vec<Subscription>,
}

impl StatusBar {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .px_4()
            .py_1()
            .bg(cx.theme().muted)
            .w_full()
            .items_start()
            .gap_2()
            .child(
                div()
                    .flex()
                    .gap_4()
                    .h_full()
                    .items_start()
                    .child(IconName::Network)
                    .child(IconName::BatteryCharging),
            )
            .child(Divider::vertical().color(cx.theme().accent))
            .child(div().w_full())
            .child(Divider::vertical().color(cx.theme().accent))
            .child(
                div()
                    .flex()
                    .gap_4()
                    .h_full()
                    .items_start()
                    .child(DpIconName::Bug),
            )
    }
}
