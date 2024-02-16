pub mod pacman;

pub fn manager_from_distro(distro: &str) -> String {
    match distro.to_lowercase().as_str() {
        "arch" => "pacman".to_string(),
        _ => panic!("Cannot find package manager for {}", distro),
    }
}

pub fn packages_from_manager(manager: &str) -> Vec<String> {
    match manager {
        "pacman" => pacman::get_packages(),
        _ => panic!("Manager for tou distro not implemented yet !"),
    }
}
