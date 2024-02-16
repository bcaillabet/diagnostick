mod managers;
pub mod packages;

use self::packages::Packages;

pub fn get_packages() -> Packages {
    Packages::new("arch").get_installed_packages()
}
