use self::linux::Linux;

pub mod linux;

pub fn detect_linux() -> Linux {
    Linux::new().parse_os_release().parse_uname()
}
