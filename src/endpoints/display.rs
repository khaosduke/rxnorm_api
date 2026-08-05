use crate::{errors::RxNormError, RxNormApi};
use std::collections::HashMap;

impl RxNormApi {
     pub async fn display_terms(
         &self,
         options:&HashMap<&str,&str>
     ) -> Result<reqwest::Response, RxNormError> {
        let function = "getDisplayTerms";

        let response= self.get(function,&options).await?;
        Ok(response)

     }
 }