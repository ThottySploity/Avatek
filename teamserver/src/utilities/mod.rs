use std::path::Path;
use std::fs;

use log::error;

pub struct Utils;

impl Utils {

    pub fn is_path_valid(path: &str) -> bool {
        // Validates if a given path exists on the machine
        Path::new(&path).exists()
    }

    pub fn create_path(path: &str) -> bool {
        // Creates a directory
        if let Err(e) = fs::create_dir_all(path) {
            error!("Failed creating path: {} with error: {}", path, e);
            return false;
        }
        true
    }
}