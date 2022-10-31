use crate::plugin::{cli::*, ApplyDotfiles};
use dip::bevy::{
    app::{App, Plugin},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::ParallelSystemDescriptorCoercion,
    },
};

pub struct DotfilesHandlerPlugin;

impl Plugin for DotfilesHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DotfilesActionPlugin)
            .add_system(apply.label("apply_dotfiles"));
    }
}

// Systems

fn apply(mut actions: EventReader<ApplyDotfilesAction>, mut apply: EventWriter<ApplyDotfiles>) {
    for _action in actions.iter() {
        apply.send(ApplyDotfiles);
    }
}
