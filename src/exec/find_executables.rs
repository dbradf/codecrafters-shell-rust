use std::{env, fs, path::Path};

use is_executable::IsExecutable;

pub fn find_executables_in_path() -> Vec<String> {
    if let Ok(paths_to_search) = env::var("PATH") {
        return env::split_paths(&paths_to_search)
            .flat_map(|dir| find_executables_in_dir(&dir))
            .collect();
    }

    vec![]
}

fn find_executables_in_dir(dir: &Path) -> Vec<String> {
    if !dir.is_dir() {
        return vec![];
    }

    fs::read_dir(dir)
        .unwrap()
        .filter_map(|f| {
            let f = f.unwrap().path();
            if f.exists() && f.is_executable() {
                Some(f.file_name().unwrap().to_str().unwrap().to_string())
            } else {
                None
            }
        })
        .collect()
}
