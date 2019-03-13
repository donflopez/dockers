extern crate dockers;
extern crate serde_json;
use dockers::{containers::ContainerConfig, Container};

use serde_json::json;
use std::collections::HashMap;

#[test]
fn test() {
    let all_containers = Container::list().unwrap();
    println!("{:?}", all_containers);

    let res = Container::new(None, Some("debian:jessie".to_owned()))
        .create(Some("my_cont_test".to_owned()), None)
        .unwrap();
    println!("{:?}", res);

    let deleted = res.remove().unwrap();
    println!("Container deleted: {:?}", deleted);
}

#[test]
fn create_with_config() {
    let img = "debian:jessie".to_owned();
    let mut exposed_ports = HashMap::new();

    exposed_ports.insert("22/tcp".to_owned(), json!({}));

    let container_conf = ContainerConfig {
        Image: img.clone(),
        ExposedPorts: Some(exposed_ports),
        ..Default::default()
    };

    let container = Container::new(None, Some(img))
        .create(Some("my_cont_test_port".to_owned()), Some(container_conf))
        .unwrap();
    println!("{:?}", container);

    let deleted = container.remove().unwrap();
    println!("Container deleted: {:?}", deleted);
}
