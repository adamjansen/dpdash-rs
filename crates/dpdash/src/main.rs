use egui_extras::install_image_loaders;

use clap::Parser;

use eframe::egui;

#[derive(Parser, Debug)]
#[command(name = "dpdash", disable_version_flag = true, max_term_width = 100)]
struct Args {
    // Set custom directory for all user data.
    // Overrides default platform-specific data directory location.
    // macOS: `~/Library/Application Support/DPDash`
    // Linux: `$XDG_DATA_HOME/DPDash`
    // Windows: `%LOCALAPPDATA%\DPDash`
    #[arg(long, value_name = "DIR", verbatim_doc_comment)]
    user_data_dir: Option<String>,
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([640.0, 480.0])
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "DPDash",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(dpdash::DpDashApp::default()))
        }),
    )
}
