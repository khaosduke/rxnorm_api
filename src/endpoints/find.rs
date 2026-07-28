use crate::{errors::RxNormError, RxNormApi};

impl RxNormApi {
     pub async fn find_rxcui_by_string(
         &self,
         name: &str,
     ) -> Result<reqwest::Response, RxNormError> {

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