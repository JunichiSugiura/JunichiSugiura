mod cli;
mod handler;

use crate::plugin::cli::{ActionPlugin, CliPlugin};
use dip::{
    bevy::app::{App, Plugin},
    core::task::NoAsyncAction,
};

use self::handler::HandlerPlugin;

pub struct DotfilesPlugin;

impl Plugin for DotfilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CliPlugin::<NoAsyncAction>::application())
            .add_plugin(ActionPlugin)
            .add_plugin(HandlerPlugin);
    }
}
