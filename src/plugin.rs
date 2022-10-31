pub mod cli;
pub mod config;
pub mod event;
pub mod tool;

use cli::{Action, ActionPlugin, CliPlugin};
use config::ConfigPlugin;
use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::{
            event::{EventReader, EventWriter},
            schedule::ParallelSystemDescriptorCoercion,
        },
    },
    core::task::NoAsyncAction,
};
use event::*;
use tool::ToolPlugin;

pub struct DotfilesPlugin;

impl Plugin for DotfilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InstallDotfiles>()
            .add_event::<ApplyDotfiles>()
            .add_plugin(ConfigPlugin)
            .add_plugin(CliPlugin::<NoAsyncAction>::application())
            .add_plugin(ActionPlugin)
            .add_plugin(ToolPlugin)
            .add_system(action_handler.label("dotfiles"));
    }
}

fn action_handler(
    mut actions: EventReader<Action>,
    mut install: EventWriter<InstallDotfiles>,
    mut apply: EventWriter<ApplyDotfiles>,
) {
    for action in actions.iter() {
        match action {
            Action::Install => {
                install.send(InstallDotfiles);
            }
            Action::Apply => {
                apply.send(ApplyDotfiles);
            }
        }
    }
}
