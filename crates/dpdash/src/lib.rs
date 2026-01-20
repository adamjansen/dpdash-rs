#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod content;
pub mod icons;
pub mod sidebar;
pub mod theme_provider;

pub use app::{AppTheme, DpDashApp};
pub use content::render_main_content;
pub use icons::draw_gear_icon;
pub use sidebar::render_sidebar;
pub use theme_provider::SimpleThemeProvider;
