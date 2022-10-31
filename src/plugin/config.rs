use dip::bevy::app::{App, Plugin};
use std::{fs, path::PathBuf};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Config>();
    }
}

pub struct Config {
    app_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_path: dirs::home_dir().unwrap().join(".dip"),
        }
    }
}

impl Config {
    pub fn app_path(&self) -> PathBuf {
        Self::ensure_dir(&self.app_path);

        self.app_path.clone()
    }

    pub fn install_path(&self) -> PathBuf {
        let p = self.app_path().join("installs");
        Self::ensure_dir(&p);

        p
    }

    fn ensure_dir(p: &PathBuf) {
        if !&p.is_dir() {
            fs::create_dir_all(&p).unwrap();
        }
    }
}
