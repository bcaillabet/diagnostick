use serde::{Deserialize, Serialize};

use super::managers::{manager_from_distro, packages_from_manager};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packages {
    manager: String,
    installed: Vec<String>,
}

impl ToString for Packages {
    fn to_string(&self) -> String {
        format!("{}", self.manager)
    }
}

impl Packages {
    pub fn new(distro: &str) -> Packages {
        Packages {
            manager: manager_from_distro(distro),
            installed: Vec::new(),
        }
    }

    pub fn get_installed_packages(mut self) -> Self {
        self.installed = packages_from_manager(&self.manager);
        self
    }
}

#[cfg(test)]
#[path = "tests/packages_tests.rs"]
mod packages_tests;
