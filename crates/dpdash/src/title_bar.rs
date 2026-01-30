use std::rc::Rc;

use gpui::{
    AnyElement, App, Context, Entity, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement as _, Render, SharedString, Styled as _, Subscription, Window, div,
};
use gpui_component::{TitleBar, label::Label, menu::AppMenuBar};

use crate::app_menus;

type ChildFunc = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;

pub struct AppTitleBar {
    app_menu_bar: Entity<AppMenuBar>,
    child: ChildFunc,
    _subscriptions: Vec<Subscription>,
}

impl AppTitleBar {
    pub fn new(
        title: impl Into<SharedString>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let app_menu_bar = app_menus::init(title, cx);

        Self {
            app_menu_bar,
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

impl Render for AppTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new()
            .child(div().flex().items_center().child(self.app_menu_bar.clone()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_2()
                    .gap_2()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child((self.child.clone())(window, cx))
                    .child(Label::new(env!("CARGO_PKG_VERSION")).text_sm()),
            )
    }
}
