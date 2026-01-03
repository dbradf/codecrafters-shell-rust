use std::{env, path::PathBuf};

use is_executable::IsExecutable;

pub fn search_path(command: &str) -> Option<PathBuf> {
    if let Ok(paths_to_search) = env::var("PATH") {
        for path in env::split_paths(&paths_to_search) {
            let maybe_path = path.join(command);
            if maybe_path.exists() && maybe_path.is_executable() {
                return Some(maybe_path);
            }
        }
    }

    None
}
