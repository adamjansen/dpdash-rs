use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, ParentElement, Render, Styled,
    Subscription, Window, px,
};

use gpui_component::{dock::PanelControl, input::*, *};

use crate::Tool;

pub struct ConsoleTool {
    cmd_input: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
    focus_handle: FocusHandle,
}

impl ConsoleTool {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let cmd_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter command here…")
                .multi_line(false)
                .clean_on_escape()
        });
        let _subscriptions = vec![cx.subscribe_in(&cmd_input, window, Self::on_input_event)];

        Self {
            cmd_input,
            _subscriptions,
            focus_handle: cx.focus_handle(),
        }
    }

    fn on_input_event(
        &mut self,
        _state: &Entity<InputState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change => {}
            InputEvent::PressEnter { secondary } => {
                // Secondary will be true if CTRL was pressed
                println!("PressEnter secondary: {}", secondary);
                self.cmd_input.update(cx, |this, cx| {
                    this.set_value("", window, cx);
                })
            }
            InputEvent::Focus => {}
            InputEvent::Blur => {}
        }
    }
}

impl Tool for ConsoleTool {
    fn title() -> &'static str {
        "Console"
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

impl Focusable for ConsoleTool {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ConsoleTool {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .size_full()
            .justify_start()
            .gap_3()
            .child(Input::new(&self.cmd_input).cleanable(true))
    }
}
