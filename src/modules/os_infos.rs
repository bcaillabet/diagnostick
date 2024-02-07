use std::{
    fs::read_to_string,
    process::{Command, Output},
};

#[derive(Debug)]
pub struct Linux {
    hostname: String,
    distro_name: String,
    version: String,
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
    fn new() -> Linux {
        Linux {
            hostname: "".to_string(),
            distro_name: "".to_string(),
            version: "".to_string(),
        }
    }

    fn parse_os_release(mut self) -> Self {
        read_to_string("/etc/os-release")
            .unwrap()
            .lines()
            .for_each(|line| {
                if line.starts_with("NAME=") {
                    match line.strip_prefix("NAME=") {
                        Some(x) => self.distro_name = x.to_string(),
                        None => self.distro_name = "Undefined".to_string(),
                    }
                }
            });

        self
    }

    fn parse_uname(mut self) -> Self {
        let output: Output = Command::new("uname")
            .arg("-a")
            .output()
            .expect("Error while trying run uname -a");

        if output.status.code() != Some(0) {
            self.hostname = "undefined".to_string();
            self.version = "undefined".to_string();
        } else {
            let output_str: String = String::from_utf8(output.stdout).unwrap();
            let output_splited: Vec<&str> = output_str.split(" ").collect();

            self.hostname = output_splited[1].to_string();
            self.version = output_splited[2].to_string();
        }

        self
    }
}

pub fn detect() -> Linux {
    let linux: Linux = Linux::new().parse_os_release().parse_uname();

    linux
}