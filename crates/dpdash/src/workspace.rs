use anyhow::{Context as _, Result};

use gpui::*;
use gpui_component::{
    IconName, Root, Sizable,
    button::{Button, ButtonVariants as _},
    dock::{ClosePanel, DockArea, DockAreaState, DockEvent, DockItem, DockPlacement, ToggleZoom},
    menu::DropdownMenu,
};

use crate::{AppState, Open, title_bar::AppTitleBar};

use serde::Deserialize;
use std::{sync::Arc, time::Duration};

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct AddPanel(DockPlacement);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct TogglePanelVisible(SharedString);

actions!(workspace, [ToggleDockToggleButton]);

const MAIN_DOCK_AREA: DockAreaTab = DockAreaTab {
    id: "main-dock",
    version: 1,
};

#[cfg(debug_assertions)]
const STATE_FILE: &str = "target/docks.json";
#[cfg(not(debug_assertions))]
const STATE_FILE: &str = "docks.json";

pub fn init(cx: &mut App) {
    cx.on_action(|_action: &Open, _cx: &mut App| {});

    cx.bind_keys(vec![
        KeyBinding::new("shift-escape", ToggleZoom, None),
        KeyBinding::new("ctrl-w", ClosePanel, None),
    ]);

    cx.activate(true);
}

pub struct Workspace {
    title_bar: Entity<AppTitleBar>,
    dock_area: Entity<DockArea>,
    last_layout_state: Option<DockAreaState>,
    toggle_button_visible: bool,
    _save_layout_task: Option<Task<()>>,
}

struct DockAreaTab {
    id: &'static str,
    version: usize,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let dock_area =
            cx.new(|cx| DockArea::new(MAIN_DOCK_AREA.id, Some(MAIN_DOCK_AREA.version), window, cx));
        let weak_dock_area = dock_area.downgrade();

        match Self::load_layout(dock_area.clone(), window, cx) {
            Ok(_) => {
                println!("Load layout OK");
            }
            Err(err) => {
                eprintln!("Load layout failed: {:?}", err);
                Self::reset_default_layout(weak_dock_area, window, cx);
            }
        };

        cx.subscribe_in(
            &dock_area,
            window,
            |this, dock_area, ev: &DockEvent, window, cx| match ev {
                DockEvent::LayoutChanged => this.save_layout(dock_area, window, cx),
                _ => {}
            },
        )
        .detach();

        cx.on_app_quit({
            let dock_area = dock_area.clone();
            move |_, cx| {
                let state = dock_area.read(cx).dump(cx);
                cx.background_executor().spawn(async move {
                    Self::save_state(&state).unwrap();
                })
            }
        })
        .detach();

        let title_bar = cx.new(|cx| {
            AppTitleBar::new("DPDash", window, cx).child({
                move |_, cx| {
                    Button::new("add-panel")
                        .icon(IconName::LayoutDashboard)
                        .small()
                        .ghost()
                        .dropdown_menu({
                            let invisible_panels = AppState::global(cx).invisible_panels.clone();

                            move |menu, _, cx| {
                                menu.menu(
                                    "Add Panel to Center",
                                    Box::new(AddPanel(DockPlacement::Center)),
                                )
                                .separator()
                                .menu("Add Panel to Left", Box::new(AddPanel(DockPlacement::Left)))
                                .menu(
                                    "Add Panel to Right",
                                    Box::new(AddPanel(DockPlacement::Right)),
                                )
                                .menu(
                                    "Add Panel to Bottom",
                                    Box::new(AddPanel(DockPlacement::Bottom)),
                                )
                                .separator()
                                .menu(
                                    "Show / Hide Dock Toggle Button",
                                    Box::new(ToggleDockToggleButton),
                                )
                                .separator()
                                .menu_with_check(
                                    "Sidebar",
                                    !invisible_panels
                                        .read(cx)
                                        .contains(&SharedString::from("Sidebar")),
                                    Box::new(TogglePanelVisible(SharedString::from("Sidebar"))),
                                )
                                .menu_with_check(
                                    "Dialog",
                                    !invisible_panels
                                        .read(cx)
                                        .contains(&SharedString::from("Dialog")),
                                    Box::new(TogglePanelVisible(SharedString::from("Dialog"))),
                                )
                                .menu_with_check(
                                    "Accordion",
                                    !invisible_panels
                                        .read(cx)
                                        .contains(&SharedString::from("Accordion")),
                                    Box::new(TogglePanelVisible(SharedString::from("Accordion"))),
                                )
                                .menu_with_check(
                                    "List",
                                    !invisible_panels
                                        .read(cx)
                                        .contains(&SharedString::from("List")),
                                    Box::new(TogglePanelVisible(SharedString::from("List"))),
                                )
                            }
                        })
                        .anchor(Corner::TopRight)
                }
            })
        });

        Self {
            dock_area,
            title_bar,
            last_layout_state: None,
            toggle_button_visible: true,
            _save_layout_task: None,
        }
    }

    fn save_layout(
        &mut self,
        dock_area: &Entity<DockArea>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let dock_area = dock_area.clone();
        self._save_layout_task = Some(cx.spawn_in(window, async move |story, window| {
            window
                .background_executor()
                .timer(Duration::from_secs(10))
                .await;

            _ = story.update_in(window, move |this, _, cx| {
                let dock_area = dock_area.read(cx);
                let state = dock_area.dump(cx);

                let last_layout_state = this.last_layout_state.clone();
                if Some(&state) == last_layout_state.as_ref() {
                    return;
                }

                Self::save_state(&state).unwrap();
                this.last_layout_state = Some(state);
            });
        }));
    }

    fn save_state(state: &DockAreaState) -> Result<()> {
        println!("Saving layout");
        let json = serde_json::to_string_pretty(state)?;
        std::fs::write(STATE_FILE, json)?;
        Ok(())
    }

    fn load_layout(
        dock_area: Entity<DockArea>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let json = std::fs::read_to_string(STATE_FILE)?;
        let state = serde_json::from_str::<DockAreaState>(&json)?;

        if state.version != Some(MAIN_DOCK_AREA.version) {
            let answer = window.prompt(
                PromptLevel::Info,
                "The default main layout has been changed.\n\
            Do you want to reset the layout to default?",
                None,
                &["Yes", "No"],
                cx,
            );

            let weak_dock_area = dock_area.downgrade();
            cx.spawn_in(window, async move |this, window| {
                if answer.await == Ok(0) {
                    _ = this.update_in(window, |_, window, cx| {
                        Self::reset_default_layout(weak_dock_area, window, cx);
                    });
                }
            })
            .detach();
        }

        dock_area.update(cx, |dock_area, cx| {
            dock_area.load(state, window, cx).context("load layout")?;
            dock_area.set_dock_collapsible(
                Edges {
                    left: true,
                    bottom: true,
                    right: true,
                    ..Default::default()
                },
                window,
                cx,
            );

            Ok::<(), anyhow::Error>(())
        })
    }

    fn reset_default_layout(dock_area: WeakEntity<DockArea>, window: &mut Window, cx: &mut App) {
        let dock_item = Self::init_default_layout(&dock_area, window, cx);

        let left_panels = DockItem::v_split(vec![], &dock_area, window, cx);

        let bottom_panels = DockItem::v_split(
            vec![DockItem::tabs(vec![], &dock_area, window, cx)],
            &dock_area,
            window,
            cx,
        );

        let right_panels = DockItem::v_split(vec![], &dock_area, window, cx);

        _ = dock_area.update(cx, |view, cx| {
            view.set_version(MAIN_DOCK_AREA.version, window, cx);
            view.set_center(dock_item, window, cx);
            view.set_left_dock(left_panels, Some(px(350.)), true, window, cx);
            view.set_bottom_dock(bottom_panels, Some(px(200.)), true, window, cx);
            view.set_right_dock(right_panels, Some(px(330.)), true, window, cx);

            Self::save_state(&view.dump(cx)).unwrap();
        });
    }

    fn init_default_layout(
        dock_area: &WeakEntity<DockArea>,
        window: &mut Window,
        cx: &mut App,
    ) -> DockItem {
        DockItem::v_split(
            vec![DockItem::tabs(vec![], &dock_area, window, cx)],
            &dock_area,
            window,
            cx,
        )
    }

    pub fn new_local(cx: &mut App) -> Task<anyhow::Result<WindowHandle<Root>>> {
        let mut window_size = size(px(1600.), px(1200.0));
        if let Some(display) = cx.primary_display() {
            let display_size = display.bounds().size;
            window_size.width = window_size.width.min(display_size.width * 0.85);
            window_size.height = window_size.height.min(display_size.height * 0.85);
        }

        let window_bounds = Bounds::centered(None, window_size, cx);
        cx.spawn(async move |cx| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                #[cfg(not(target_os = "linux"))]
                titlebar: Some(gpui_component::TitleBar::title_bar_options()),
                window_min_size: Some(gpui::Size {
                    width: px(640.),
                    height: px(480.),
                }),
                #[cfg(target_os = "linux")]
                window_background: gpui::WindowBackgroundAppearance::Transparent,
                #[cfg(target_os = "linux")]
                window_decorations: Some(gpui::WindowDecorations::Client),
                kind: WindowKind::Normal,
                ..Default::default()
            };

            let window = cx.open_window(options, |window, cx| {
                let story_view = cx.new(|cx| Workspace::new(window, cx));
                cx.new(|cx| Root::new(story_view, window, cx))
            })?;

            window
                .update(cx, |_, window, cx| {
                    window.activate_window();
                    window.set_window_title("GPUI App");
                    cx.on_release(|_, cx| {
                        // exit app
                        cx.quit();
                    })
                    .detach();
                })
                .expect("failed to update window");

            Ok(window)
        })
    }

    fn on_action_add_panel(
        &mut self,
        action: &AddPanel,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {

        // Create a panel
        // let panel = Arc::new(ToolContainer::panel::<SomeTool>(window, cx));
        // self.dock_area.update(cvx, |dock_area, cx| { dock_area.add_panel(panel, action.0, None, window, cx);});
    }

    fn on_action_toggle_panel_visible(
        &mut self,
        action: &TogglePanelVisible,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let panel_name = action.0.clone();
        let invisible_panels = AppState::global(cx).invisible_panels.clone();
        invisible_panels.update(cx, |names, cx| {
            if names.contains(&panel_name) {
                names.retain(|id| id != &panel_name);
            } else {
                names.push(panel_name);
            }
            cx.notify();
        });
        cx.notify();
    }

    fn on_action_toggle_dock_toggle_button(
        &mut self,
        _: &ToggleDockToggleButton,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_button_visible = !self.toggle_button_visible;

        self.dock_area.update(cx, |dock_area, cx| {
            dock_area.set_toggle_button_visible(self.toggle_button_visible, cx);
        });
    }
}
pub fn open_new(
    cx: &mut App,
    init: impl FnOnce(&mut Root, &mut Window, &mut Context<Root>) + 'static + Send,
) -> Task<()> {
    let task: Task<std::result::Result<WindowHandle<Root>, anyhow::Error>> =
        Workspace::new_local(cx);
    cx.spawn(async move |cx| {
        if let Some(root) = task.await.ok() {
            root.update(cx, |workspace, window, cx| init(workspace, window, cx))
                .expect("failed to init workspace");
        }
    })
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("workspace")
            .on_action(cx.listener(Self::on_action_add_panel))
            .on_action(cx.listener(Self::on_action_toggle_panel_visible))
            .on_action(cx.listener(Self::on_action_toggle_dock_toggle_button))
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .child(self.title_bar.clone())
            .child(self.dock_area.clone())
    }
}
