extern crate dockers;

use dockers::Container;

use std::process::exit;

#[test]
fn test() {
    let all_containers = Container::list().unwrap();
    println!("{:?}", all_containers);

    let mut cmd: Vec<String> = Vec::new();
    cmd.push("ls".to_string());
    let res = Container::new(None, Some("debian:jessie".to_owned()))
        .create(Some("my_cont_test".to_owned()), None)
        .unwrap();
    println!("{:?}", res);

    let deleted = res.remove().unwrap();
    println!("Container deleted: {:?}", deleted);
}
