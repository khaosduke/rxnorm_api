use crate::{errors::RxNormError, RxNormApi};
use std::collections::HashMap;
use rxnorm_api::validators::verify_options_hash;

impl RxNormApi {
     pub async fn find_rxcui_by_string(
         &self,
         name: &str,
         options: &HashMap<&str,&str>,
     ) -> Result<reqwest::Response, RxNormError> {

        let mut working_options = options.clone();
        let _ = working_options.insert("name",name);

        verify_options_hash("findRxcuiByString",&working_options)?;
        

     }
//
//     pub async fn find_rxcui_by_id(
//         &self,
//         id: &str,
//     ) -> Result<reqwest::Response, RxNormError> {
//
//     }
//
//     pub async fn find_remapped(
//         &self,
//         rxcui: &str,
//     ) -> Result<reqwest::Response, RxNormError> {
//
//     }
}