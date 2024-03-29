use crate::executor::EngineConfigurationStrategy;

use rhai::{Engine, EvalAltResult};

pub struct FileCatConfiguration;

// impl EngineConfigurationStrategy for FileCatConfiguration {
//     fn configure_engine(&self, engine: &mut Engine) {
//         engine.register_fn(
//             "cat_files",
//             |dir_path: &str| -> Result<String, Box<EvalAltResult>> {
//                 let mut result = String::new();
//                 let paths = std::fs::read_dir(dir_path)
//                     .map_err(|e| format!("Error reading directory: {}", e))?;

//                 for path in paths {
//                     let path = path
//                         .map_err(|e| format!("Error reading path: {}", e))?
//                         .path();
//                     if path.is_file() {
//                         let content = std::fs::read_to_string(&path)
//                             .map_err(|e| format!("Error reading file {:?}: {}", path, e))?;
//                         result.push_str(&content);
//                     }
//                 }
//                 Ok(result)
//             },
//         );
//     }
// }

impl EngineConfigurationStrategy for FileCatConfiguration {
    fn configure_engine(&self, engine: &mut Engine) {
        engine.register_fn(
            "cat_files",
            |dir_path: &str| -> Result<String, Box<EvalAltResult>> {
                let mut result = String::new();
                let paths = std::fs::read_dir(dir_path)
                    .map_err(|e| format!("Error reading directory: {}", e))?;

                for path in paths {
                    let path = path
                        .map_err(|e| format!("Error reading path: {}", e))?
                        .path();
                    if path.is_file() {
                        // Get the file name as a string
                        let file_name = path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("Unknown File")
                            .to_owned();

                        // Append the file name to the result string
                        result.push_str(&format!("\nFile: {}\n", file_name));

                        let content = std::fs::read_to_string(&path)
                            .map_err(|e| format!("Error reading file {:?}: {}", path, e))?;
                        result.push_str(&content);
                    }
                }
                Ok(result)
            },
        );
    }
}
