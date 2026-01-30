use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, ParentElement, Render, Styled as _,
    Window, div, px,
};

use gpui_component::{
    IconName,
    dock::PanelControl,
    label::Label,
    stepper::{Stepper, StepperItem},
};

use crate::Tool;
use crate::icons::DpIconName;

pub struct DpLoadTool {
    focus_handle: FocusHandle,
    step: usize,
}

impl DpLoadTool {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            step: 0,
        }
    }
}

impl Tool for DpLoadTool {
    fn title() -> &'static str {
        "Programming Tool"
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

impl Focusable for DpLoadTool {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DpLoadTool {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex()
            .flex_col()
            .items_start()
            .justify_between()
            .child(
                Stepper::new("stepper1")
                    .text_2xl()
                    .w_full()
                    .selected_index(self.step)
                    .items([
                        StepperItem::new().icon(IconName::Settings).child("Setup"),
                        StepperItem::new()
                            .icon(IconName::Network)
                            .child("Scan")
                            .disabled(self.step > 0),
                        StepperItem::new()
                            .icon(DpIconName::Binary)
                            .child("Program")
                            .disabled(self.step > 1),
                        StepperItem::new()
                            .icon(IconName::CircleCheck)
                            .disabled(self.step > 2)
                            .child("Verify"),
                        StepperItem::new()
                            .icon(IconName::Info)
                            .disabled(self.step > 3)
                            .child("Finish"),
                    ]),
            )
            .child(match self.step {
                0 => Label::new("step 1"),
                1 => Label::new("step 2"),
                2 => Label::new("step 3"),
                3 => Label::new("step 4"),
                4 => Label::new("step 5"),
                _ => unreachable!(),
            })
    }
}
