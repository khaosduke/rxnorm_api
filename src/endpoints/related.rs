use crate::{errors::RxNormError, RxNormApi};
use std::collections::HashMap;


impl RxNormApi {
     pub async fn get_related_by_type(
         &self,
         rxcui: &str,
         options: &HashMap<&str,&str>
     ) -> Result<reqwest::Response, RxNormError> {
        
        let function = "getRelatedByType";

        let mut working_options = options.clone();
        let _ = working_options.insert("rxcui",rxcui);
        
        let response= self.get(function,&working_options).await?;
        
        Ok(response)
     }
//
//     pub async fn get_all_related_info(
//         &self,
//         rxcui: &str,
//     ) -> Result<reqwest::Response, RxNormError> {
//
//     }
//
//     pub async fn get_all_concepts_by_tty(
//         &self,
//         tty: &str,
//     ) -> Result<reqwest::Response, RxNormError> {
//
//     }
}