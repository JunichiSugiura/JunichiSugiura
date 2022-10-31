mod cli;
mod handler;
mod homebrew;

use crate::plugin::cli::{ActionPlugin, CliPlugin};
use dip::{
    bevy::app::{App, Plugin},
    core::task::NoAsyncAction,
};

use self::{handler::HandlerPlugin, homebrew::HomebrewPlugin};

// Events

pub struct InstallDotfiles;

pub struct ApplyDotfiles;

pub struct DotfilesPlugin;

impl Plugin for DotfilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstallDotfiles>()
            .add_event::<ApplyDotfiles>()
            .add_plugin(CliPlugin::<NoAsyncAction>::application())
            .add_plugin(ActionPlugin)
            .add_plugin(HandlerPlugin)
            .add_plugin(HomebrewPlugin);
    }
}
