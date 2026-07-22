use rxnorm_api::add;
use rxnorm_api::build_get_request;
use reqwest::Client;
use std::time::Duration;
use std::collections::HashMap;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = add(5, 3);
    println!("The result is: {}", result);

    let client = Client::builder()
        // Set a timeout for the entire request
        .timeout(Duration::from_secs(30))
        // Set a timeout for establishing connections
        .connect_timeout(Duration::from_secs(10))
        //Library
        .user_agent("RxNorm_API/1.0")
        // Build the client
        .build()?;

    
    //let request_url = "/REST/rxcui.xml?name=yourName&allsrc=0or1&srclist=yourSources&search=0or1or2or9";

    let response = client
        .get("https://httpbin.org/")
        .send()
        .await?;    

    //println!("Response: {:?}", response.text().await?);

    let test_opt = HashMap::from([
        ("foo","bar"),
        ("fizz","buzz")
    ]);

    let test_opt_str = build_get_request("test",&test_opt);
    println!("{:?}",test_opt_str);

    Ok(())
}