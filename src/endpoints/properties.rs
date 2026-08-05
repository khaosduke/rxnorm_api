use crate::{errors::RxNormError, RxNormApi};
use std::collections::HashMap;


impl RxNormApi {
     pub async fn get_all_properties(
         &self,
         rxcui: &str,
         options: &HashMap<&str,&str>,
     ) -> Result<reqwest::Response, RxNormError> {
        
        let function = "getAllProperties";

        let mut working_options = options.clone();
        let _ = working_options.insert("rxcui",rxcui);

        let response= self.get(function,&working_options).await?;
        
        Ok(response)
     }


    pub async fn get_rx_concept_properties(
        &self,
        rxcui: &str,
        options: &HashMap<&str,&str>,
    ) -> Result<reqwest::Response, RxNormError> {
        
        let function = "getRxConceptProperties";
        let mut working_options = options.clone();

        let _ = working_options.insert("rxcui", rxcui);

        let response= self.get(function,&working_options).await?;

        Ok(response)
    }
}
//
//     pub async fn get_property(
//         &self,
//         rxcui: &str,
//         property: &str,
//     ) -> Result<reqwest::Response, RxNormError> {
//
//     }
//
//     pub async fn get_multi_ingred_brand(
//         &self,
//         rxcui: &str,
//     ) -> Result<reqwest::Response, RxNormError> {
//
//     }
