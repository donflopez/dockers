use docker::{Client, Method};

pub trait Info: Client {
    fn get_info(&self) -> Result<String, String> {
        let api_endpoint = "/info";
        let method = Method::GET;

        Ok(self
            .get_response_from_api(api_endpoint, method, None, None)
            .unwrap()
            .body)
    }
}
