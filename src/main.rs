use dip::bevy::app::App;
use dotfiles::DotfilesPlugin;

fn main() {
    App::new().add_plugin(DotfilesPlugin).run();
}
