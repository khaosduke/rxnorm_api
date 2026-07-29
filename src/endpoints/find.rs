use crate::{errors::RxNormError, RxNormApi};
use std::collections::HashMap;
use crate::validators::verify_options_hash;


impl RxNormApi {
     pub async fn find_rxcui_by_string(
         &self,
         name: &str,
         options: &HashMap<&str,&str>,
     ) -> Result<reqwest::Response, RxNormError> {

        let mut working_options = options.clone();
        let _ = working_options.insert("name",name);
        //Can check that function name is correct but it should be if youre writing it within the API
        let function = "findRxcuiByString";
        //Everythings valid
        verify_options_hash(function,&working_options)?;
        let response= self.get(function,&working_options).await?;
        
        Ok(response)
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