extern crate dockers;
extern crate serde_json;
use dockers::{
    containers::{ContainerConfig, HostConfig, PortBinding},
    Container,
};

use serde_json::json;
use std::{collections::HashMap, thread, time::Duration};

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
    let mut published_ports = HashMap::new();

    published_ports.insert(
        "22/tcp".to_owned(),
        vec![PortBinding {
            HostPort: "22".to_owned(),
            HostIp: "0.0.0.0".to_owned(),
        }],
    );

    let container_conf = ContainerConfig {
        Image: img.clone(),
        HostConfig: HostConfig {
            PortBindings: Some(published_ports),
            ..Default::default()
        },
        ..Default::default()
    };

    let container = Container::new(None, Some(img))
        .create(Some("my_cont_test_port".to_owned()), Some(container_conf))
        .unwrap();

    let deleted = container.remove().unwrap();
    println!("Container deleted: {:?}", deleted);
}
