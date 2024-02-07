use std::net::{SocketAddr, ToSocketAddrs};

/*
Testing localhost resolution
*/
let mut addrs_iter = "localhost:443".to_socket_addrs().unwrap();
assert_eq!(addrs_iter.next(), Some(SocketAddr::from(([127, 0, 0, 1], 443))));

/*
Testing internet(google?) DNS resolution
*/
let mut addrs_iter = "google.com:443".to_socket_addrs().unwrap();
assert_eq!(addrs_iter.next(), Some(SocketAddr::from(([127, 0, 0, 1], 443))));

/*
Check DNS configuration on linux (resolv.conf file) ?
*/