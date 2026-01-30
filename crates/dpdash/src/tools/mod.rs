use gpui::{AnyView, App, AppContext as _, Entity, Hsla, Pixels, Render, Window, px};
use gpui_component::dock::PanelControl;

mod console_tool;
mod device_manager_tool;
mod dpload_tool;
mod example_tool;
mod property_editor;

pub use console_tool::ConsoleTool;
pub use device_manager_tool::DeviceManagerTool;
pub use dpload_tool::DpLoadTool;
pub use example_tool::ExampleTool;
pub use property_editor::PropertyEditorTool;

pub(crate) fn init(_cx: &mut App) {
    // If any tools require explicit initialization, do it here
}

pub trait Tool: Render + Sized {
    fn klass() -> &'static str {
        std::any::type_name::<Self>().split("::").last().unwrap()
    }

    fn title() -> &'static str;

    fn description() -> &'static str {
        ""
    }

    fn closable() -> bool {
        true
    }

    fn zoomable() -> Option<PanelControl> {
        Some(PanelControl::default())
    }

    fn title_bg() -> Option<Hsla> {
        None
    }

    fn paddings() -> Pixels {
        px(16.)
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render>;

    fn on_active(&mut self, active: bool, window: &mut Window, cx: &mut App) {
        let _ = active;
        let _ = window;
        let _ = cx;
    }

    fn on_active_any(view: AnyView, active: bool, window: &mut Window, cx: &mut App)
    where
        Self: 'static,
    {
        if let Ok(tool) = view.downcast::<Self>() {
            cx.update_entity(&tool, |tool, cx| {
                tool.on_active(active, window, cx);
            });
        }
    }
}
