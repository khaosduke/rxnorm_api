use crate::{errors::RxNormError, RxNormApi};

impl RxNormApi {
     pub async fn get_id_types(
         &self,
         options: &std::collections::HashMap<&str,&str>
     ) -> Result<reqwest::Response, RxNormError> {
        
        let function = "getIdTypes";

        let response= self.get(function,&options).await?;
        Ok(response)
     }

//     pub async fn get_rxcui_for_id(
//         &self,
//         id: &str,
//         id_type: &str,
//     ) -> Result<reqwest::Response, RxNormError> {
//
//     }
}