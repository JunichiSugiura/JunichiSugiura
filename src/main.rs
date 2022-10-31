use dip::bevy::{app::App, log::LogPlugin};
use dotfiles::DotfilesPlugin;

fn main() {
    App::new()
        .add_plugin(DotfilesPlugin)
        .add_plugin(LogPlugin)
        .run();
}
