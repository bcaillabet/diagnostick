use std::fs::read_to_string;

pub struct Dns {
    resolve_localhost: bool,
    resolve_google: bool,
    dns_server: String,
    local_domain: String
}


impl Dns {
    pub fn new() -> Dns {
        Dns {
            resolve_localhost: false, // adding false as it needs a boolean value
            resolve_google: false, // otherwise errors pop up during build
            dns_server: "".to_string(),
            local_domain: "".to_string(),
        }
    }

    pub fn parse_resolve_conf(mut self) -> Self {
        read_to_string("/etc/resolve.conf")
            .unwrap()
            .lines()
            .for_each(|line| {
                if line.starts_with("nameserver") {
                    match line.strip_prefix("nameserver ") {
                        Some(s) => self.dns_server = s.to_string(),
                        None => self.dns_server = "Undefined".to_string(),
                    }
                }
            });

        self
    }
}

/*
Testing localhost resolution
*/


/*
Testing internet(google?) DNS resolution
*/
/*
Check DNS configuration on linux (resolv.conf file) ?
*/

