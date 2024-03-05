pub mod linux;

use self::linux::Linux;

pub fn detect_linux() -> Linux {
    Linux::new().parse_os_release().parse_uname().users()
}
