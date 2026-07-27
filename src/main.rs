use rxnorm_api::build_get_request;
use reqwest::Client;
use std::time::Duration;
use std::collections::HashMap;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    let client = Client::builder()
        // Set a timeout for the entire request
        .timeout(Duration::from_secs(30))
        // Set a timeout for establishing connections
        .connect_timeout(Duration::from_secs(10))
        //Library
        .user_agent("RxNorm_API/1.0")
        // Build the client
        .build()?;

    


    let test_opt = HashMap::from([
        ("rxcui","1012407"),
        //("foo","bs")
        ("format","json")
        //("propName","RXNAV_STR")
    ]);

    let function = "findActiveProducts";
    //let function = "foo";    
    let test_opt_str = build_get_request(function,&test_opt)?;

    println!("{:?}",test_opt_str.to_string());

    let response = client
        .get(test_opt_str)
        .send()
        .await?;    


    println!("RESPONSE: {:?}",response.text().await?);
    
    //Concept
    let drug = "fentanyl";
    let drug_rxcui = "4337";

    let drug_function = "findRxcuiByString";
    let drug_related_by_type_function = "getRelatedByType";

    let findrxcui_ops = HashMap::from([
        ("name",drug),
        ("format","json"),
        ("search","2")
    ]);

    let relatedbytype_ops = HashMap::from([
        ("rxcui",drug_rxcui),
        ("format","json"),
        ("tty","SCD SBD SCDG SBDG")
    ]);

    let drug_resp = client
        .get(build_get_request(drug_function,&findrxcui_ops)?)
        .send()
        .await?;
    println!("Drug Response: {:?}",drug_resp.text().await?);    



    let related_str = build_get_request(drug_related_by_type_function,&relatedbytype_ops)?;
    println!("TO GET: {:?}",related_str.to_string());
    let relatedbytype_resp = client
        .get(related_str)
        .send()
        .await?;
    println!("Related by type: {:?}",relatedbytype_resp.text().await?);
    
    







    Ok(())
}