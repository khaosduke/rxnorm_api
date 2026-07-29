use rxnorm_api::RxNormApi;
use std::collections::HashMap;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
 
    let rxnorm = RxNormApi::new()?;

    let drug_rxcui = "4337";//Fentanyl


    let relatedbytype_ops = HashMap::from([
        ("format","json"),
        ("tty","SCD SBD SCDG SBDG")
    ]);

    //let response = rxnorm.get(drug_related_by_type_function,&relatedbytype_ops).await?;
    let response = rxnorm.get_related_by_type(drug_rxcui, &relatedbytype_ops).await?;
    println!("Got: {:?}",response.text().await?);
    

    Ok(())
}