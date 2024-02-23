pub mod dns;
use self::dns::Dns;

pub fn get_dns_state() -> Dns {
    Dns::new().parse_resolve_conf()
}