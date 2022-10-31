use crate::plugin::{cli::*, ApplyDotfiles, InstallDotfiles};
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
            .add_system(install.label("install_dotfiles"))
            .add_system(apply.label("apply_dotfiles"));
    }
}

// Systems

fn install(
    mut actions: EventReader<InstallDotfilesAction>,
    mut install: EventWriter<InstallDotfiles>,
) {
    for _action in actions.iter() {
        install.send(InstallDotfiles);
    }
}

fn apply(mut actions: EventReader<ApplyDotfilesAction>, mut apply: EventWriter<ApplyDotfiles>) {
    for _action in actions.iter() {
        apply.send(ApplyDotfiles);
    }
}
