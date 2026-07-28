use rxnorm_api::RxNormApi;
use std::collections::HashMap;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
 
    let rxnorm = RxNormApi::new()?;

    let drug_rxcui = "4337";//Fentanyl

    let drug_related_by_type_function = "getRelatedByType";

    let relatedbytype_ops = HashMap::from([
        ("rxcui",drug_rxcui),
        ("format","json"),
        ("tty","SCD SBD SCDG SBDG")
    ]);

    let response = rxnorm.get(drug_related_by_type_function,&relatedbytype_ops).await?;
    println!("Got: {:?}",response.text().await?);
    

    Ok(())
}