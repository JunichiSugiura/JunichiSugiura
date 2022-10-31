use dip::bevy::{app::App, log::LogPlugin};
use dip_bundle::BundlePlugin;

fn main() {
    App::new()
        .add_plugin(BundlePlugin)
        .add_plugin(LogPlugin)
        .run();
}
