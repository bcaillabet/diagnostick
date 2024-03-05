use std::{
    fs::read_to_string,
    process::{Command, Output},
};

use serde::{Deserialize, Serialize};
use users::{get_user_by_uid, get_current_uid};

#[derive(Serialize, Deserialize, Debug)]
pub struct Linux {
    hostname: String,
    distro_name: String,
    version: String,
    username: String,
    uid: String,
    gid: String,
}

impl ToString for Linux {
    fn to_string(&self) -> String {
        format!(
            "{} [{} ({})]",
            self.hostname, self.distro_name, self.version
        )
    }
}

impl Linux {
    pub fn new() -> Linux {
        Linux {
            hostname: "".to_string(),
            distro_name: "".to_string(),
            version: "".to_string(),
            username: "".to_string(),
            uid: "".to_string(),
            gid: "".to_string(),
        }
    }

    pub fn parse_os_release(mut self) -> Self {
        read_to_string("/etc/os-release")
            .unwrap()
            .lines()
            .for_each(|line| {
                if line.starts_with("NAME=") {
                    match line.strip_prefix("NAME=") {
                        Some(s) => self.distro_name = s.to_string(),
                        None => self.distro_name = "Undefined".to_string(),
                    }
                }
            });

        self
    }

    pub fn parse_uname(mut self) -> Self {
        let output: Output = Command::new("uname")
            .arg("-a")
            .output()
            .expect("Error while trying run uname -a");

        if output.status.code() != Some(0) {
            self.hostname = "undefined".to_string();
            self.version = "undefined".to_string();
        } else {
            let output_str: String = String::from_utf8(output.stdout).unwrap();
            let fields: Vec<&str> = output_str.split(" ").collect();

            self.hostname = fields[1].to_string();
            self.version = fields[2].to_string();
        }

        self
    }

    pub fn users(mut self) -> Self {
        match get_user_by_uid(get_current_uid()) {
            Some(user) => {
                self.username = user.name().to_string_lossy().to_string();
                self.uid = user.uid().to_string();
                self.gid = user.primary_group_id().to_string();
            },
            None => {
                // Will be updating here to handle errors
                // In case no current user is found
            }
        }

        self
    }
}

#[cfg(test)]
#[path = "tests/linux_tests.rs"]
mod linux_tests;
