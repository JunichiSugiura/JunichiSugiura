use std::{
    fs::{self, Permissions},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

pub struct Path;

impl Path {
    fn app_path() -> PathBuf {
        let p = dirs::home_dir().unwrap().join(".dip");
        Self::ensure_dir(&p);
        p
    }

    fn install_path() -> PathBuf {
        let p = Self::app_path().join("installs");
        Self::ensure_dir(&p);
        p
    }

    fn ensure_dir(p: &PathBuf) {
        if !&p.is_dir() {
            fs::create_dir_all(&p).unwrap();
        }
    }
}
