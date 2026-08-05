use crate::{errors::RxNormError, RxNormApi};

impl RxNormApi {
     pub async fn get_rxcui_history_status(
         &self,
         rxcui: &str,
         options: &std::collections::HashMap<&str,&str>
     ) -> Result<reqwest::Response, RxNormError> {
        let function = "getRxcuiHistoryStatus";

        let mut working_options = options.clone();
        let _ = working_options.insert("rxcui",rxcui);

        let response= self.get(function,&working_options).await?;
        Ok(response)
     }

     pub async fn get_all_historical_ndcs(
         &self,
         rxcui: &str,
         options: &std::collections::HashMap<&str,&str>
     ) -> Result<reqwest::Response, RxNormError> {
        let function = "getAllHistoricalNDCs";

        let mut working_options = options.clone();
        let _ = working_options.insert("rxcui",rxcui);

        let response= self.get(function,&working_options).await?;
        Ok(response)

     }
 }