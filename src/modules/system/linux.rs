use std::{
    fs::read_to_string,
    process::{Command, Output},
};

use serde::{Deserialize, Serialize};
use users::{all_users, get_user_by_uid, get_current_uid, get_effective_uid, get_effective_gid};

#[derive(Serialize, Deserialize, Debug)]
pub struct Linux {
    hostname: String,
    distro_name: String,
    version: String,
    username: String,
    uid: String,
    euid: String,
    gid: String,
    egid: String,
    service_users: String,
    system_users: String,
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
            euid: "".to_string(),
            gid: "".to_string(),
            egid: "".to_string(),
            service_users: "".to_string(),
            system_users: "".to_string(),
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

        // Retrieving current user information (username,uid, euid, gid, egid)
        match get_user_by_uid(get_current_uid()) {
            Some(user) => {
                self.username = user.name().to_string_lossy().to_string();
                self.uid = user.uid().to_string();
                self.euid = get_effective_uid().to_string(); //euid is not part of the user method so it needs the fn here 
                self.gid = user.primary_group_id().to_string();
                self.egid = get_effective_gid().to_string(); //egid, same applies here
            },
            None => {
                // Will be updating here to handle errors
                // In case no current user is found
            }
        }

        // Retrieving service and system users count
        let mut service_users_cnt = 0;
        let mut system_users_cnt = 0;
        let iter = unsafe { all_users()};
        for user in iter {
            if user.uid() < 1000 {
                service_users_cnt += 1;
            }
            else if user.uid() >= 1000 && user.uid() < 65534 {
                system_users_cnt += 1;
            }
        }
        self.service_users = service_users_cnt.to_string();
        self.system_users = system_users_cnt.to_string();

        self
    }
}

#[cfg(test)]
#[path = "tests/linux_tests.rs"]
mod linux_tests;
