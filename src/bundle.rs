pub mod cli;
pub mod config;
pub mod tool;

use cli::{ActionPlugin, ApplyAction, CliPlugin};
use config::BundleConfigPlugin;
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
use tool::ToolPlugin;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyBundle>()
            .add_plugin(BundleConfigPlugin)
            .add_plugin(CliPlugin::<NoAsyncAction>::application())
            .add_plugin(ActionPlugin)
            .add_plugin(ToolPlugin)
            .add_system(apply_bundle.label("apply_bundle"));
    }
}

// Events

pub struct ApplyBundle {
    pub action: ApplyAction,
}

// Systems

fn apply_bundle(mut actions: EventReader<ApplyAction>, mut apply: EventWriter<ApplyBundle>) {
    for action in actions.iter() {
        apply.send(ApplyBundle {
            action: action.clone(),
        });
    }
}
