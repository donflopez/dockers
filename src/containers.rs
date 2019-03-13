#![allow(non_snake_case)]

//! Containers is the main structure for handling containers
//!
//! You can create an empty container or you can pass the ID of an existing
//! one or an Image name to create a new one.
//!
//! # Example
//!
//! ```rust
//! extern crate dockers;
//!
//! use dockers::Container;
//! use dockers::Image;
//!
//! let image = Image::pull("node".to_owned(), None)
//!     .expect("Image not pulled");
//!
//! // Create a container struct from an image and then create the actual container.
//! let cont = Container::new(None, Some("node".to_owned()));
//! let cont = cont.create(None, None).expect("Couldn't create node container");
//!
//! // Clean up things to not alter the state of the running docker
//! cont.remove().expect("Couldn't remove the container");
//! image.remove().expect("Couldn't remove the image");
//! ```
use docker::DockerApiError;
use std::collections::HashMap;

use serde_json;

use docker::{get_response_from_api_static, invalid_api_resp, Client, Method};

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    pub PrivatePort: u32,
    pub PublicPort: u32,
    pub Type: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HostConfig {
    pub NetworkMode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mounts {
    #[serde(default)]
    pub Name: Option<String>,
    pub Source: String,
    pub Destination: String,
    #[serde(default)]
    pub Driver: String,
    pub Mode: String,
    pub RW: bool,
    pub Propagation: String,
}

///
/// # Container
///
/// Container struct that implement several methods for handling
/// containers.
///
/// You have multiple ways to create a container.
///
/// ```rust
/// extern crate dockers;
///
/// # use dockers::Image;
/// # let image = Image::pull("debian".to_owned(), None).unwrap();
///
/// use dockers::Container;
///
/// // From the struct
/// let cont = Container {
///     Image: "debian".to_owned(),
///     ..Default::default()
/// }
///     .create(Some("my_container_name".to_owned()), None)
///     .expect("Cannot create container for debian");
///
/// // You could clone a container!
/// // TODO
///
/// cont.remove().unwrap();
/// # image.remove().unwrap();
/// ```
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Container {
    pub Id: String,

    #[serde(default)]
    pub Names: Vec<String>,
    pub Image: String,
    pub ImageID: String,
    pub Command: String,
    pub State: String,
    pub Status: String,
    pub Ports: Vec<Port>,
    pub Labels: Option<HashMap<String, String>>,

    #[serde(default)]
    pub SizeRw: Option<i64>,

    #[serde(default)]
    pub SizeRootFs: u64,
    pub HostConfig: HostConfig,
    pub Mounts: Vec<Mounts>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContainerConfig {
    pub Image: String,
    pub Cmd: Vec<String>,

    pub Hostname: String,
    pub Domainname: String,
    pub User: String,
    pub AttachStdin: bool,
    pub AttachStdout: bool,
    pub AttachStderr: bool,
    pub Tty: bool,
    pub OpenStdin: bool,
    pub StdinOnce: bool,
    pub Env: Vec<String>,
    pub Entrypoint: Option<String>,
    pub Labels: Option<HashMap<String, String>>,
    pub WorkingDir: String,
    pub NetworkingDisabled: bool,
    pub MacAddress: String,
    pub ExposedPorts: Option<HashMap<String, String>>,
}

impl Client for Container {}

impl Container {
    pub fn new(Id: Option<String>, Image: Option<String>) -> Container {
        Container {
            Id: Id.unwrap_or_default(),
            Image: Image.unwrap_or_default(),
            ..Default::default()
        }
    }

    pub fn create(
        &self,
        name: Option<String>,
        config: Option<ContainerConfig>,
    ) -> Result<Container, DockerApiError> {
        let endpoint = "/containers/create";
        let params = format!("?name={}", name.unwrap_or_default());
        let params = Some(params.as_ref());

        let conf = match config {
            Some(conf) => ContainerConfig {
                Image: if conf.Image.is_empty() {
                    self.Image.clone()
                } else {
                    conf.Image
                },
                ..conf
            },
            None => ContainerConfig {
                Image: self.Image.clone(),
                ..Default::default()
            },
        };

        let body = serde_json::to_string(&conf).expect("Can't parse container configuration");

        let body = Some(body.as_ref());

        let res = self.get_response_from_api(endpoint, Method::POST, params, body)?;

        if res.status_code != 201 {
            return Err(DockerApiError::InvalidApiResponseError(
                res.status_code,
                res.body,
            ));
        }

        serde_json::from_str(&res.body).map_err(|e| DockerApiError::JsonDeserializationError(e))
    }

    pub fn list() -> Result<Vec<Container>, DockerApiError> {
        let endpoint = "/containers/json";
        let params = "?all=true&size=true";

        let res = get_response_from_api_static(endpoint, Method::GET, Some(params), None)?;

        if res.status_code != 200 {
            return Err(DockerApiError::InvalidApiResponseError(
                res.status_code,
                res.body,
            ));
        }

        serde_json::from_str(&res.body).map_err(|e| DockerApiError::JsonDeserializationError(e))
    }

    fn container_action(&self, action: &str) -> Result<String, DockerApiError> {
        let endpoint = format!("/containers/{}/{}", self.Id, action);

        let res = self.get_response_from_api(&endpoint, Method::POST, None, None)?;

        if res.status_code != 204 {
            return Err(invalid_api_resp(res));
        }

        Ok(format!("Operation {} run correctly", action))
    }

    pub fn start(&self) -> Result<String, DockerApiError> {
        self.container_action("start")
    }

    pub fn stop(&self) -> Result<String, DockerApiError> {
        self.container_action("stop")
    }

    pub fn restart(&self) -> Result<String, DockerApiError> {
        self.container_action("restart")
    }

    pub fn kill(&self) -> Result<String, DockerApiError> {
        self.container_action("kill")
    }

    pub fn pause(&self) -> Result<String, DockerApiError> {
        self.container_action("pause")
    }

    pub fn unpause(&self) -> Result<String, DockerApiError> {
        self.container_action("unpause")
    }

    pub fn remove(self) -> Result<String, DockerApiError> {
        let endpoint = format!("/containers/{}", self.Id);

        let res = self.get_response_from_api(&endpoint, Method::DELETE, None, None)?;

        if res.status_code != 204 {
            return Err(invalid_api_resp(res));
        }

        Ok("Container deleted correctly".to_owned())
    }
}
