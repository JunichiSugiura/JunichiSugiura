use crate::bundle::ApplyBundle;
use cmd_lib::{run_fun, spawn_with_output};
use dip::bevy::{
    app::{App, AppExit, Plugin},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::ParallelSystemDescriptorCoercion,
    },
    log,
};
use std::{
    env,
    io::{BufRead, BufReader},
};

pub struct HomebrewPlugin;

impl Plugin for HomebrewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(install.after("apply_bundle").before(apply))
            .add_system(apply.after("apply_bundle"));
    }
}

// Systems

fn install(mut events: EventReader<ApplyBundle>, mut app_exit: EventWriter<AppExit>) {
    for ApplyBundle { action } in events.iter() {
        log::warn!("TODO: change current_path to somewhere absolute");

        let current_path = env::current_dir().expect("Failed to get current directory.");
        let brewfile_path = current_path
            .join("bundles")
            .join("homebrew")
            .join("Brewfile");

        if brewfile_path.is_file() {
            match run_fun!(which brew) {
                Ok(brew_path) => {
                    if action.verbose {
                        log::info!("{brew_path}");
                    }
                    log::info!("✅ Install Homebrew");
                }
                Err(e) => {
                    if action.verbose {
                        log::warn!("{e}");
                    }
                    log::info!("📌 Install Homebrew");

                    let install_script = reqwest::blocking::get(
                        "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh",
                    )
                    .expect("Failed to fetch Homebrew installation script")
                    .text()
                    .expect("Failed to parse Homebrew installation script into text");

                    log::info!("{install_script:?}");

                    log::warn!("TODO: Run install.sh");
                    // match run_fun!(/bin/bash -C "path/to/install.sh") {
                    //     Ok(res) => {
                    //         println!("{res}");
                    //         println!("Apply homebrew bundle finished.");
                    //     }
                    //     Err(e) => {
                    //         log::error!("{e}");
                    //     }
                    // }

                    log::info!("✅ Install Homebrew");
                }
            }
        }

        app_exit.send(AppExit);
    }
}

fn apply(mut events: EventReader<ApplyBundle>, mut app_exit: EventWriter<AppExit>) {
    for ApplyBundle { action } in events.iter() {
        log::warn!("TODO: change current_path to somewhere absolute");
        let current_path = env::current_dir().expect("Failed to get current directory.");
        let brewfile_path = current_path
            .join("bundles")
            .join("homebrew")
            .join("Brewfile");

        if brewfile_path.is_file() {
            match run_fun!(which brew) {
                Ok(_brew_path) => {
                    log::info!("📌 Apply Homebrew bundle");

                    let brewfile_path_str = &brewfile_path.into_os_string().into_string().unwrap();
                    let mut brew_bundle =
                        spawn_with_output!(brew bundle --file $brewfile_path_str).unwrap();

                    if action.verbose {
                        brew_bundle
                            .wait_with_pipe(&mut |pipe| {
                                BufReader::new(pipe)
                                    .lines()
                                    .filter_map(|line| line.ok())
                                    .for_each(|line| log::info!("{:?}", line));
                            })
                            .unwrap();
                    } else {
                        if let Err(e) = brew_bundle.wait_with_output() {
                            log::error!("{e}");
                            log::error!("Failed to run brew bundle.");
                        } else {
                            log::info!("✅ Apply Homebrew bundle");
                        }
                    }
                }
                Err(e) => {
                    log::error!("{e}");
                    log::error!("Could not find homebrew binary.");
                }
            }
        } else {
            log::error!(
                "Failed to apply Homebrew bundle. Make sure to have bundles/homebrew/Brewfile"
            );
        }

        app_exit.send(AppExit);
    }
}
