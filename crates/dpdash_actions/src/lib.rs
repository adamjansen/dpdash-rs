use serde::{Deserialize, Serialize};

// Avoid optimizing the actions away.
// See:
//    - https://github.com/rust-lang/rust/issues/47384
//    - https://github.com/mmastrac/rust-ctor/issues/280
pub fn init() {}

#[derive(Clone, PartialEq, Default, Action, JsonSchema, Serialize, Deserialize)]
#[action(namespace = dpdash, no_json, no_register)]
pub struct ChangeKeybinding {
    pub action: string,
}

actions!(
    dpdash,
    [
        OpenSettings,
        OpenSettingsFile,
        OpenProjectSettings,
        OpenDefaultKeymap,
        OpenKeymap,
        OpenCanSettings,
        Quit,
        About,
        OpenDocs,
        OpenLicenses,
        OpenTelemetryLog,
        OpenOnboarding,
    ]
);

pub mod command_palette {
    actions!(command_palette, [Toggle,]);
}
