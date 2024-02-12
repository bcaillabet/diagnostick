/* Check for users :
  - Get current user information [_]
  - Get the count of svc users (id < 1000) [_]
  - Get the count of sys users (id > 1000) [_]

  Linux only
*/

use users::{get_current_uid,get_current_username,get_effective_gid,get_effective_groupname,get_effective_uid};
use std::cmp::Ordering;

fn current_user() {
   
    // Function to retrieve current user information 
    // uid, effective uid, gid, effective gid

    let user_uid = get_current_uid();
    let user_eid = get_effective_uid();
    
    match get_current_username() {
        Some(uname) => println!("Running as user: {:?}", uname),
        None        => println!("Running as user: current user does not exist"),
    }

    match user_uid.cmp(&user_eid) {
    Ordering::Equal => println!("Current user id matches effective id: {user_uid}"),
    _ => {
        println!("Current user id: {user_uid}");
        println!("Current user effective id: {user_eid}");
        }
    }
    //let user_hostname = get_user_by_by_hostname(get_current_username());
    //let user_uid = get_user_by_uid(get_current_uid()).unwrap();
}

fn main() {
    current_user();
}
