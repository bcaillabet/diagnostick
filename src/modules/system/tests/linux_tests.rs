#[cfg(test)]
pub mod linux_tests {
    use crate::Linux;

    #[test]
    fn parse_os_release_update_struct() {
        let l = Linux::new().parse_os_release();
        assert_ne!(l.distro_name, "".to_string());
        assert!(1 == 1);
    }

    #[test]
    fn parse_uname_update_struct() {
        let l = Linux::new().parse_uname();

        assert_ne!(l.hostname, "".to_string());
        assert_ne!(l.version, "".to_string());
    }
}
