use gpui::{
    Action, AnyElement, AnyView, App, AppContext, Bounds, Context, Div, Entity, EventEmitter,
    FocusHandle, Focusable, Global, Hsla, InteractiveElement, IntoElement, KeyBinding,
    ParentElement, Pixels, Render, RenderOnce, SharedString, Size, StyleRefinement, Styled, Window,
    WindowBounds, WindowKind, WindowOptions, actions, div, prelude::FluentBuilder as _, px, rems,
    size,
};
use gpui_component::{
    ActiveTheme, IconName, Root, TitleBar, WindowExt,
    button::Button,
    dock::{Panel, PanelControl, PanelEvent, PanelInfo, PanelState, TitleStyle, register_panel},
    group_box::{GroupBox, GroupBoxVariants as _},
    h_flex,
    menu::PopupMenu,
    notification::Notification,
    scroll::{ScrollableElement as _, ScrollbarShow},
    v_flex,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod app_menus;
mod themes;
mod title_bar;
pub mod workspace;
pub use crate::title_bar::AppTitleBar;

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = story, no_json)]
pub struct SelectScrollbarShow(ScrollbarShow);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = story, no_json)]
pub struct SelectLocale(SharedString);

actions!(story, [About, Open, Quit, Tab, TabPrev,]);

pub struct AppState {
    pub invisible_panels: Entity<Vec<SharedString>>,
}

impl AppState {
    fn init(cx: &mut App) {
        let state = Self {
            invisible_panels: cx.new(|_| Vec::new()),
        };
        cx.set_global::<AppState>(state);
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }
}

impl Global for AppState {}

pub fn init(cx: &mut App) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("gpui=trace".parse().unwrap())
                .add_directive("gpui_component=trace".parse().unwrap())
                .add_directive("dpdash=trace".parse().unwrap()),
        )
        .init();

    gpui_component::init(cx);
    AppState::init(cx);
    themes::init(cx);
    workspace::init(cx);
}
