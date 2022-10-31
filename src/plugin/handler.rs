mod dotfiles;

use crate::plugin::handler::dotfiles::DotfilesHandlerPlugin;
use dip::bevy::app::{App, Plugin};

pub struct HandlerPlugin;

impl Plugin for HandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DotfilesHandlerPlugin);
    }
}
