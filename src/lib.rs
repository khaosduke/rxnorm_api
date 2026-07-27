use reqwest::Client;
use reqwest::Response;

mod request;
mod constants;
mod validators;

pub mod errors;
pub use request::build_get_request;

pub struct RxNormApi {
    client: reqwest::Client,
}

impl RxNormApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    //pub fn get(function:&str, options&HashMap<&str,&str>) -> reqwest::Response {
        
    //}
}

