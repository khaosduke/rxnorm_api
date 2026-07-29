use crate::{errors::RxNormError, RxNormApi};
use std::collections::HashMap;

impl RxNormApi {
    pub async fn get_ndcs(
        &self,
        rxcui:&str,
        options:&HashMap<&str,&str>,
    ) -> Result<reqwest::Response, RxNormError> {
        let function = "getNDCs";

        let mut working_options = options.clone();
        let _ = working_options.insert("rxcui",rxcui);

        let response= self.get(function,&working_options).await?;
    }
//     pub async fn display_terms(
//         &self,
//     ) -> Result<reqwest::Response, RxNormError> {
//
//     }
//
//     pub async fn display_names(
//         &self,
//     ) -> Result<reqwest::Response, RxNormError> {
//
//     }
}