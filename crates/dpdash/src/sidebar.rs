use crate::app::DpDashApp;
use eframe::egui;

pub fn render_sidebar(app: &mut DpDashApp, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical()
        .max_width(ui.available_width())
        .show(ui, |ui| {
            ui.add_space(6.0);
            ui.colored_label(app.get_text_color(ui), "theme");
            ui.separator();

            ui.checkbox(&mut app.show_sidebar, "show sidebar");
        });
}
