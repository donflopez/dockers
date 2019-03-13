#![allow(non_snake_case)]
use serde_json;
use std::collections::HashMap;

use docker::DockerApiError;
use docker::{get_response_from_api_static, invalid_api_resp, Client, Method};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Image {
    pub Id: String,
    pub ParentId: String,
    pub RepoTags: Option<Vec<String>>,
    pub RepoDigests: Option<Vec<String>>,
    pub Created: u64,
    pub Size: u64,
    pub VirtualSize: u64,
    pub SharedSize: i64,
    pub Labels: Option<HashMap<String, String>>,
    pub Containers: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageStatus {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletedImage {
    pub Untagged: Option<String>,
    pub Deleted: Option<String>,
}

pub enum Source {
    Image,
    Src,
    Repo,
}

impl Client for Image {}

/// Container is the main structure for handling containers
/// You can create an empty container or you can pass the ID of an existing
/// one or the Image to create a new one.
///
/// # Example
///
/// ```rust
/// extern crate dockers;
///
/// use dockers::Image;
///
/// let image = Image::pull("postgres".to_owned(), None)
///     .expect("Image not pulled");
///
/// image.remove().expect("Couldn't remove the image");
/// ```

impl Image {
    pub fn create(
        source: Source,
        value: String,
        tag: Option<String>,
    ) -> Result<Image, DockerApiError> {
        let api_endpoint = "/images/create";
        let param = match source {
            Source::Image => &"?fromImage=",
            Source::Src => &"?fromSrc=",
            Source::Repo => &"?repo=",
        };
        let tag = if tag.is_some() {
            format!("&tag={}", tag.unwrap())
        } else {
            "&tag=latest".to_owned()
        };
        let params = format!("{}{}{}", param, value, tag);
        let query_params: Option<&str> = Some(&params);

        let res =
            get_response_from_api_static(api_endpoint, Method::POST, query_params, None).unwrap();

        if res.status_code != 200 {
            return Err(invalid_api_resp(res));
        }

        Ok(Image {
            Id: value,
            ..Default::default()
        })
    }

    pub fn pull(image: String, tag: Option<String>) -> Result<Image, DockerApiError> {
        Image::create(Source::Image, image, tag)
    }

    pub fn list() -> Result<Vec<Image>, DockerApiError> {
        let api_endpoint = "/images/json";

        let res = get_response_from_api_static(api_endpoint, Method::GET, None, None).unwrap();

        if res.status_code != 200 {
            return Err(invalid_api_resp(res));
        }

        serde_json::from_str(&res.body).map_err(DockerApiError::JsonDeserializationError)
    }

    pub fn remove(self) -> Result<Vec<DeletedImage>, DockerApiError> {
        let api_endpoint = format!("/images/{}", self.Id);
        let res = self.get_response_from_api(&api_endpoint, Method::DELETE, None, None)?;

        if res.status_code != 200 {
            return Err(invalid_api_resp(res));
        }

        serde_json::from_str(&res.body).map_err(DockerApiError::JsonDeserializationError)
    }
}
