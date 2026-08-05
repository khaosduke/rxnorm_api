use reqwest::Client;
use std::time::Duration;
use std::collections::HashMap;
use crate::errors::RxNormError;

pub use request::build_request;


mod request;
mod constants;
mod validators;
mod endpoints;

pub mod errors;

#[derive(Debug)]
pub struct RxNormApi {
    client: reqwest::Client,
}

impl RxNormApi {
    pub fn new() -> Result<Self, RxNormError>  {
         let client = Client::builder()
            // Set a timeout for the entire request
            .timeout(Duration::from_secs(30))
            // Set a timeout for establishing connections
            .connect_timeout(Duration::from_secs(10))
            //Library
            .user_agent("RxNorm_API/1.0")
            // Build the client
            .build()?;
        
            Ok(Self { client })
    }
    //pub(crate) async fn get(...)
    async fn get( &self, function: &str,  options:&HashMap<&str,&str>,)
                    -> Result<reqwest::Response, RxNormError> {

        let request_url = build_request(function,&options)?;
        println!("TO GET: {:?}",request_url.to_string());
        
        let response = self.client
        .get(request_url)
        .send()
        .await?
        .error_for_status()?;              
    
        Ok(response)
    }

}

