use crate::app::DpDashApp;

pub fn render_main_content(app: &mut DpDashApp, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical()
        .max_width(ui.available_width())
        .show(ui, |ui| {
            ui.colored_label(app.get_text_color(ui), "asdf");
            ui.separator();

            ui.label("howdy partner");
            ui.add_space(16.0);
        });
}
