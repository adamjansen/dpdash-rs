use gpui::Application;
use gpui_component_assets::Assets;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        dpdash::init(cx);
        dpdash::workspace::open_new(cx, |_, _, _| {
            // do something
        })
        .detach();
    });
}
