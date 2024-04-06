use crate::executor::EngineConfigurationStrategy;
use rhai::{Engine, EvalAltResult};
use std::fs::{self, DirEntry};
use std::path::Path;

pub struct FileCatConfiguration;

impl EngineConfigurationStrategy for FileCatConfiguration {
    fn configure_engine(&self, engine: &mut Engine) {
        engine.register_fn("cat_files", cat_files);
    }
}

fn cat_files(dir_path: &str, recursive: bool) -> Result<String, Box<EvalAltResult>> {
    let mut result = String::new();
    let path = Path::new(dir_path);
    if path.is_dir() {
        visit_dirs(path, &mut result, recursive)?;
    }
    Ok(result)
}

fn visit_dirs(dir: &Path, result: &mut String, recursive: bool) -> Result<(), Box<EvalAltResult>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).map_err(|e| format!("Error reading directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Error reading path: {}", e))?;
            let path = entry.path();
            if path.is_dir() && recursive {
                visit_dirs(&path, result, recursive)?;
            } else if path.is_file() {
                append_file_contents(&entry, result)?;
            }
        }
    }
    Ok(())
}

fn append_file_contents(entry: &DirEntry, result: &mut String) -> Result<(), Box<EvalAltResult>> {
    let path = entry.path();
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown File");
    let content =
        fs::read_to_string(&path).map_err(|e| format!("Error reading file {:?}: {}", path, e))?;

    result.push_str(&format!("\nFile: {}\n", file_name));
    result.push_str(&content);

    Ok(())
}
