pub mod info;
pub mod errors;

use curl::easy::{Easy, List};
use std::env::var;

pub use self::errors::DockerApiError;

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: usize,
    pub body: String,
}

pub enum Method {
    GET,
    POST,
    DELETE,
}

fn request(
    api_endpint: &str,
    method: Method,
    body_w: Option<&str>,
) -> Response {
    let unix_socket_url =
        var("DOCKERS_SOCKET_URL").unwrap_or("/var/run/docker.sock".to_owned());

    let mut e = Easy::new();
    let mut status_code: usize = 0;
    let mut body = Vec::new();
    let body_w = body_w.unwrap_or_default();

    e.unix_socket(&unix_socket_url).unwrap();

    e.url(&format!("http{}", api_endpint)).unwrap();
    let mut headers = List::new();

    headers.append("Content-Type: application/json").unwrap();

    e.http_headers(headers).unwrap();

    let _ = match method {
        Method::GET => e.get(true).unwrap(),
        Method::POST => {
            e.post(true).unwrap();
            e.post_fields_copy(body_w.as_bytes()).unwrap()
        }
        Method::DELETE => {
            e.custom_request("DELETE").unwrap();
        }
    };

    {
        let mut transfer = e.transfer();

        transfer
            .header_function(|h| {
                let header = String::from_utf8(h.to_vec())
                    .expect("Header to string failed");
                print!("header: {}", header);
                if status_code == 0 {
                    let parts: Vec<&str> = header.splitn(3, " ").collect();
                    if parts.len() > 1 {
                        status_code = parts[1]
                            .parse()
                            .expect("Cannot parse this string into usize");
                    }
                }
                true
            })
            .unwrap();

        transfer
            .write_function(|d| {
                body.extend_from_slice(d);

                Ok(d.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    let body = if body.is_empty() {
        "".to_owned()
    } else {
        String::from_utf8(body[..body.len() - 1].to_vec())
            .expect("Cannot parse vec to string")
    };

    println!("{}", body);

    Response {
        status_code: status_code.clone(),
        body,
    }
}

pub fn invalid_api_resp(res: Response) -> DockerApiError {
    DockerApiError::InvalidApiResponseError(res.status_code, res.body)
}

pub fn get_response_from_api_static(
    api_endpoint: &str,
    method: Method,
    query_params: Option<&str>,
    body: Option<&str>,
) -> Result<Response, DockerApiError> {
    let api_endpoint =
        format!("{}{}", api_endpoint, query_params.unwrap_or_default());

    Ok(request(&api_endpoint, method, body))
}

pub trait Client {
    fn get_response_from_api(
        &self,
        api_endpoint: &str,
        method: Method,
        query_params: Option<&str>,
        body: Option<&str>,
    ) -> Result<Response, DockerApiError> {
        let res: Result<Response, DockerApiError> =
            get_response_from_api_static(
                api_endpoint,
                method,
                query_params,
                body,
            );

        res
    }
}
