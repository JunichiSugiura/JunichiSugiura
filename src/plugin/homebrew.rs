use crate::plugin::ApplyDotfiles;
use dip::bevy::{
    app::{App, AppExit, Plugin},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::ParallelSystemDescriptorCoercion,
    },
};
use std::{
    env,
    process::{Command, Stdio},
};

pub struct HomebrewPlugin;

impl Plugin for HomebrewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply.after("apply_dotfiles"));
    }
}

// Systems

fn apply(mut events: EventReader<ApplyDotfiles>, mut app_exit: EventWriter<AppExit>) {
    for _e in events.iter() {
        println!("Applying homebrew bundle...");

        let current_path = env::current_dir().expect("Failed to get current directory.");
        let brewfile_path = current_path
            .join("plugins")
            .join("homebrew")
            .join("Brewfile");

        if brewfile_path.is_file() {
            let output = Command::new("which").arg("brew").output().unwrap();

            if output.status.success() {
                let output = Command::new("brew")
                    .args([
                        "bundle",
                        "--file",
                        &brewfile_path.into_os_string().into_string().unwrap(),
                    ])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .unwrap();

                if output.status.success() {
                    println!("{}", String::from_utf8(output.stdout).unwrap());
                    println!("Apply homebrew bundle finished.");
                } else {
                    eprintln!("{}", String::from_utf8(output.stderr).unwrap());
                    eprintln!("Failed to run brew bundle.");
                }
            } else {
                eprintln!("{}", String::from_utf8(output.stderr).unwrap());
                eprintln!("Could not find homebrew binary. Make sure to install first.");
            }
        } else {
            eprintln!(
                "Failed to apply Homebrew bundle. Make sure to have plugins/homebrew/Brewfile"
            );
        }

        app_exit.send(AppExit);
    }
}
