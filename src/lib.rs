pub mod config;
pub mod modules;

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use modules::packaging::get_packages;
use modules::packaging::packages::Packages;
use modules::system::{detect_linux, linux::Linux};

use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Result};

pub const BANNER: &str = r#"
    ____  _                              __  _      __
   / __ \(_)___ _____ _____  ____  _____/ /_(_)____/ /__
  / / / / / __ `/ __ `/ __ \/ __ \/ ___/ __/ / ___/ //_/
 / /_/ / / /_/ / /_/ / / / / /_/ (__  ) /_/ / /__/ ,<
/_____/_/\__,_/\__, /_/ /_/\____/____/\__/_/\___/_/|_|
              /____/
"#;

#[derive(Serialize, Deserialize)]
pub struct Diagnostick {
    system: Linux,
    packaging: Packages,
}

impl Diagnostick {
    pub fn new() -> Diagnostick {
        Diagnostick {
            system: detect_linux(),
            packaging: get_packages(),
        }
    }

    // TODO better result...
    pub fn save(self, output_path: &Path) -> Result<()> {
        create_dir_all(output_path).unwrap();
        let mut outfile: File = File::create(Path::join(output_path, "output.json")).unwrap();

        outfile
            .write_all(to_string_pretty(&self).unwrap().as_bytes())
            .unwrap();

        Ok(())
    }
}
