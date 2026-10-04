/* TODO
- [ ] function to find db path
*/
use std::env::current_dir;
use std::fs::{create_dir, exists};
use std::io::Error as IOError;
use std::path::PathBuf;

use dirs::home_dir;

const SAVE_FOLDER: &str = ".plate_spiner";
const DB_NAME: &str = "plate-spinner.duckdb";

pub enum PathError {
    UnableToReadPath,
    UnableToCreatePath,
    GeneralError,
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self) {
            PathError::UnableToReadPath => {
                write!(f, "Unable to find a suitible place to store data")
            }
            PathError::UnableToCreatePath => {
                write!(f, "Unable to create a directory")
            }
            PathError::GeneralError => {
                write!(f, "Something went wrong trying to setup a save path")
            }
        }
    }
}

fn map_path_error(_: IOError) -> PathError {
    PathError::GeneralError
}

pub fn get_a_save_path_root() -> PathBuf {
    if let Some(home) = home_dir() {
        home
    } else if let Ok(cwd) = current_dir() {
        cwd
    } else {
        let mut p = PathBuf::new();
        p.push("~/");
        p
    }
}

pub fn create_or_get_save_path() -> Result<PathBuf, PathError> {
    let mut save_path = get_a_save_path_root();
    save_path.push(SAVE_FOLDER);

    if !exists(&save_path).map_err(map_path_error)? {
        save_path.push(DB_NAME);
        Ok(save_path)
    } else {
        if create_dir(&save_path).is_ok() {
            save_path.push(DB_NAME);
            Ok(save_path)
        } else {
            Err(PathError::UnableToCreatePath)
        }
    }
}
