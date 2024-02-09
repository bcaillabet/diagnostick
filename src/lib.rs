pub mod modules;

use modules::system::{self, linux::Linux};

pub const BANNER: &str = r#"
    ____  _                              __  _      __
   / __ \(_)___ _____ _____  ____  _____/ /_(_)____/ /__
  / / / / / __ `/ __ `/ __ \/ __ \/ ___/ __/ / ___/ //_/
 / /_/ / / /_/ / /_/ / / / / /_/ (__  ) /_/ / /__/ ,<
/_____/_/\__,_/\__, /_/ /_/\____/____/\__/_/\___/_/|_|
              /____/
"#;

pub fn run_checks() {
    let linux_infos: Linux = system::detect_linux();

    println!("{}", linux_infos.to_string());
}
