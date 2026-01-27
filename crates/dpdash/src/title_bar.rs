use std::rc::Rc;

use gpui::{
    AnyElement, App, AppContext, Context, Corner, Entity, FocusHandle, InteractiveElement as _,
    IntoElement, MouseButton, ParentElement as _, Render, SharedString, Styled as _, Subscription,
    Window, div, px,
};
use gpui_component::{
    ActiveTheme as _, IconName, PixelsExt, Side, Sizable as _, Theme, TitleBar, WindowExt as _,
    badge::Badge,
    button::{Button, ButtonVariants as _},
    label::Label,
    menu::{AppMenuBar, DropdownMenu as _},
    scroll::ScrollbarShow,
};

use crate::app_menus;

pub struct AppTitleBar {
    app_menu_bar: Entity<AppMenuBar>,
    child: Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>,
    _subscriptions: Vec<Subscription>,
}

impl AppTitleBar {
    pub fn new(
        title: impl Into<SharedString>,
        window: &mut Window,
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
        let notifications_count = window.notifications(cx).len();

        TitleBar::new()
            // left side
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
                    .child(
                        Label::new("theme:")
                            .secondary(cx.theme().theme_name())
                            .text_sm(),
                    )
                    .child(
                        Button::new("github")
                            .icon(IconName::GitHub)
                            .small()
                            .ghost()
                            .on_click(|_, _, cx| {
                                cx.open_url("https://github.com/longbridge/gpui-component")
                            }),
                    )
                    .child(
                        div().relative().child(
                            Badge::new().count(notifications_count).max(99).child(
                                Button::new("bell")
                                    .small()
                                    .ghost()
                                    .compact()
                                    .icon(IconName::Bell),
                            ),
                        ),
                    ),
            )
    }
}
