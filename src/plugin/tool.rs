mod homebrew;

use dip::bevy::app::{App, Plugin};
pub use homebrew::HomebrewPlugin;

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "brew")]
        app.add_plugin(HomebrewPlugin);
    }
}
