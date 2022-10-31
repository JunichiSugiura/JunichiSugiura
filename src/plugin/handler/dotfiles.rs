use crate::plugin::cli::*;
use dip::bevy::{
    app::{App, AppExit, Plugin},
    ecs::event::{EventReader, EventWriter},
};

pub struct DotfilesHandlerPlugin;

impl Plugin for DotfilesHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DotfilesActionPlugin).add_system(apply);
    }
}

fn apply(mut actions: EventReader<ApplyDotfilesAction>, mut app_exit: EventWriter<AppExit>) {
    for action in actions.iter() {
        println!("{action:?}");

        app_exit.send(AppExit);
    }
}
