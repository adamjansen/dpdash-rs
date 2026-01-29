use gpui::{
    Action, AnyView, App, AppContext, Bounds, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Global, Hsla, InteractiveElement, IntoElement, ParentElement, Pixels, Render,
    SharedString, Size, Styled, Window, WindowBounds, WindowKind, WindowOptions, actions, div,
    prelude::FluentBuilder as _, px, size,
};
use gpui_component::{
    ActiveTheme, IconName, Root, TitleBar, WindowExt,
    button::Button,
    dock::{Panel, PanelControl, PanelEvent, PanelInfo, PanelState, TitleStyle, register_panel},
    notification::Notification,
    scroll::{ScrollableElement as _, ScrollbarShow},
    v_flex,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod app_menus;
mod themes;
mod title_bar;
mod tools;
pub mod workspace;
pub use crate::title_bar::AppTitleBar;

pub use tools::*;

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = story, no_json)]
pub struct SelectScrollbarShow(ScrollbarShow);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = story, no_json)]
pub struct SelectLocale(SharedString);

actions!(story, [About, Open, Quit, Tab, TabPrev, ShowPanelInfo]);

const PANEL_NAME: &str = "ToolContainer";

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

pub fn create_new_window<F, E>(title: &str, crate_view_fn: F, cx: &mut App)
where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    create_new_window_with_size(title, None, crate_view_fn, cx);
}

pub fn create_new_window_with_size<F, E>(
    title: &str,
    window_size: Option<Size<Pixels>>,
    crate_view_fn: F,
    cx: &mut App,
) where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    let mut window_size = window_size.unwrap_or(size(px(1600.0), px(1200.0)));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        window_size.width = window_size.width.min(display_size.width * 0.85);
        window_size.height = window_size.height.min(display_size.height * 0.85);
    }
    let window_bounds = Bounds::centered(None, window_size, cx);
    let title = SharedString::from(title.to_string());

    cx.spawn(async move |cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(gpui::Size {
                width: px(480.),
                height: px(320.),
            }),
            kind: WindowKind::Normal,
            #[cfg(target_os = "linux")]
            window_background: gpui::WindowBackgroundAppearance::Transparent,
            #[cfg(target_os = "linux")]
            window_decorations: Some(gpui::WindowDecorations::Client),
            ..Default::default()
        };

        let window = cx
            .open_window(options, |window, cx| {
                let view = crate_view_fn(window, cx);
                let story_root = cx.new(|cx| ToolRoot::new(title.clone(), view, window, cx));

                // Set focus to the StoryRoot to enable it's actions.
                let focus_handle = story_root.focus_handle(cx);
                window.defer(cx, move |window, cx| {
                    focus_handle.focus(window, cx);
                });

                cx.new(|cx| Root::new(story_root, window, cx))
            })
            .expect("failed to open window");

        window
            .update(cx, |_, window, _| {
                window.activate_window();
                window.set_window_title(&title);
            })
            .expect("failed to update window");

        Ok::<_, anyhow::Error>(())
    })
    .detach();
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
    tools::init(cx);

    cx.on_action(|_: &Quit, cx: &mut App| {
        cx.quit();
    });

    cx.on_action(|_: &About, cx: &mut App| {
        if let Some(window) = cx.active_window().and_then(|w| w.downcast::<Root>()) {
            cx.defer(move |cx| {
                window
                    .update(cx, |root, window, cx| {
                        root.open_dialog(
                            move |dialog, _, _| {
                                dialog
                                    .confirm()
                                    .child("DPDash")
                                    .child(env!("CARGO_PKG_VERSION"))
                                    .child(option_env!("DPDASH_COMMIT_SHA").unwrap_or("nope"))
                                    .child("© 2026 Data Panel Corporation")
                                    .alert()
                            },
                            window,
                            cx,
                        )
                    })
                    .unwrap();
            });
        }
    });

    register_panel(cx, PANEL_NAME, |_, _, info, window, cx| {
        let tool_state = match info {
            PanelInfo::Panel(value) => ToolState::from_value(value.clone()),
            _ => {
                unreachable!("Invalid PanelInfo: {:?}", info)
            }
        };

        let view = cx.new(|cx| {
            let (title, description, closable, zoomable, tool, on_active) =
                tool_state.to_tool(window, cx);
            let mut container = ToolContainer::new(window, cx)
                .tool(tool, tool_state.tool_klass)
                .on_active(on_active);

            cx.on_focus_in(
                &container.focus_handle,
                window,
                |this: &mut ToolContainer, _, _| {
                    println!("ToolContainer focus in: {}", this.name);
                },
            )
            .detach();

            container.name = title.into();
            container.description = description.into();
            container.closable = closable;
            container.zoomable = zoomable;
            container
        });
        Box::new(view)
    });

    cx.activate(true);
}

pub struct ToolContainer {
    focus_handle: gpui::FocusHandle,
    pub name: SharedString,
    pub title_bg: Option<Hsla>,
    pub description: SharedString,
    width: Option<gpui::Pixels>,
    height: Option<gpui::Pixels>,
    tool: Option<AnyView>,
    tool_klass: Option<SharedString>,
    closable: bool,
    zoomable: Option<PanelControl>,
    paddings: Pixels,
    on_active: Option<fn(AnyView, bool, &mut Window, &mut App)>,
}

#[derive(Debug)]
pub enum ContainerEvent {
    Close,
}

impl EventEmitter<ContainerEvent> for ToolContainer {}

impl ToolContainer {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            focus_handle,
            name: "".into(),
            title_bg: None,
            description: "".into(),
            width: None,
            height: None,
            tool: None,
            tool_klass: None,
            closable: true,
            zoomable: Some(PanelControl::default()),
            paddings: px(16.),
            on_active: None,
        }
    }

    pub fn panel<T: Tool>(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let name = T::title();
        let description = T::description();
        let tool = T::new_view(window, cx);
        let tool_klass = T::klass();

        let view = cx.new(|cx| {
            let mut tool = Self::new(window, cx)
                .tool(tool.into(), tool_klass)
                .on_active(T::on_active_any);
            tool.focus_handle = cx.focus_handle();
            tool.closable = T::closable();
            tool.zoomable = T::zoomable();
            tool.name = name.into();
            tool.description = description.into();
            tool.title_bg = T::title_bg();
            tool.paddings = T::paddings();
            tool
        });

        view
    }

    pub fn width(mut self, width: gpui::Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: gpui::Pixels) -> Self {
        self.height = Some(height);
        self
    }

    pub fn tool(mut self, tool: AnyView, tool_klass: impl Into<SharedString>) -> Self {
        self.tool = Some(tool);
        self.tool_klass = Some(tool_klass.into());
        self
    }

    pub fn on_active(mut self, on_active: fn(AnyView, bool, &mut Window, &mut App)) -> Self {
        self.on_active = Some(on_active);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolState {
    pub tool_klass: SharedString,
}

impl ToolState {
    fn to_value(&self) -> serde_json::Value {
        serde_json::json!({
            "tool_klass": self.tool_klass,
        })
    }

    fn from_value(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }

    fn to_tool(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> (
        &'static str,
        &'static str,
        bool,
        Option<PanelControl>,
        AnyView,
        fn(AnyView, bool, &mut Window, &mut App),
    ) {
        macro_rules! tool {
            ($klass:tt) => {
                (
                    $klass::title(),
                    $klass::description(),
                    $klass::closable(),
                    $klass::zoomable(),
                    $klass::view(window, cx).into(),
                    $klass::on_active_any,
                )
            };
        }

        match self.tool_klass.to_string().as_str() {
            "ExampleTool" => tool!(ExampleTool),
            "DeviceManagerTool" => tool!(DeviceManagerTool),
            "ConsoleTool" => tool!(ConsoleTool),
            _ => {
                unreachable!("Invalid tool klass: {}", self.tool_klass)
            }
        }
    }
}

impl Panel for ToolContainer {
    fn panel_name(&self) -> &'static str {
        "ToolContainer"
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.name.clone().into_any_element()
    }

    fn title_style(&self, cx: &App) -> Option<TitleStyle> {
        if let Some(bg) = self.title_bg {
            Some(TitleStyle {
                background: bg,
                foreground: cx.theme().foreground,
            })
        } else {
            None
        }
    }

    fn closable(&self, _cx: &App) -> bool {
        self.closable
    }

    fn zoomable(&self, _cx: &App) -> Option<PanelControl> {
        self.zoomable
    }

    fn visible(&self, cx: &App) -> bool {
        !AppState::global(cx)
            .invisible_panels
            .read(cx)
            .contains(&self.name)
    }

    fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, _cx: &mut Context<Self>) {
        println!("panel: {} zoomed: {}", self.name, zoomed);
    }

    fn set_active(&mut self, active: bool, _window: &mut Window, cx: &mut Context<Self>) {
        println!("panel: {} active: {}", self.name, active);
        if let Some(on_active) = self.on_active {
            if let Some(tool) = self.tool.clone() {
                on_active(tool, active, _window, cx);
            }
        }
    }

    fn toolbar_buttons(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Vec<Button>> {
        Some(vec![Button::new("info").icon(IconName::Info).on_click(
            |_, window, cx| {
                window.push_notification("clicked info button", cx);
            },
        )])
    }

    fn dump(&self, _cx: &App) -> PanelState {
        let mut state = PanelState::new(self);
        let tool_state = ToolState {
            tool_klass: self.tool_klass.clone().unwrap(),
        };
        state.info = PanelInfo::panel(tool_state.to_value());
        state
    }
}

impl EventEmitter<PanelEvent> for ToolContainer {}
impl Focusable for ToolContainer {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ToolContainer {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("tool-container")
            .size_full()
            .overflow_y_scrollbar()
            .track_focus(&self.focus_handle)
            .when_some(self.tool.clone(), |this, tool| {
                this.child(div().size_full().p(self.paddings).child(tool))
            })
    }
}

struct ToolRoot {
    focus_handle: FocusHandle,
    title_bar: Entity<AppTitleBar>,
    view: AnyView,
}

impl ToolRoot {
    pub fn new(
        title: impl Into<SharedString>,
        view: impl Into<AnyView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| AppTitleBar::new(title, window, cx));
        Self {
            focus_handle: cx.focus_handle(),
            title_bar,
            view: view.into(),
        }
    }

    fn on_action_panel_info(
        &mut self,
        _: &ShowPanelInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        struct Info;
        let note = Notification::new()
            .message("you cliekd the info")
            .id::<Info>();
        window.push_notification(note, cx);
    }
}

impl Focusable for ToolRoot {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for ToolRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .id("tool-root")
            .on_action(cx.listener(Self::on_action_panel_info))
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(self.title_bar.clone())
                    .child(
                        div()
                            .track_focus(&self.focus_handle)
                            .flex_1()
                            .overflow_hidden()
                            .child(self.view.clone()),
                    )
                    .children(sheet_layer)
                    .children(dialog_layer)
                    .children(notification_layer),
            )
    }
}
