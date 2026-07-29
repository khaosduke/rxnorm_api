use std::collections::HashMap;
use url::Url;

use crate::constants;
use crate::constants::RXNORM_FUNCTIONS;
use crate::errors::RxNormError;
use crate::validators;

pub fn build_request(function: &str, 
                             options:&HashMap<&str,&str>)
        -> Result<Url,RxNormError> {
    
    //Don't even continue if function or options are invalid
    validators::verify_function_name(function)?; 
    validators::verify_options_hash(function,options)?; 
    
    //Use Url crate to build url
    //Start with the function template stored in constants and its associated acceptable options hash
    let (path_template, _) = RXNORM_FUNCTIONS
        .get(function)
        .ok_or_else(|| RxNormError::InvalidFunction(function.to_owned()))?;        

    let mut path = path_template.to_string();   

    //Since we might change options, and they arent that big we can copy to the heap
    let mut working_options = options.clone();
        
    //Get the format
    //Add "." in its proper place
    path.push('.');
    let format = working_options
        .get("format")
        .copied()
        .unwrap_or("xml")
        .trim_start_matches('.');

    if !matches!(format, "json" | "xml") {
        return Err(RxNormError::InvalidFormat(
            format.to_owned(),
        ));
    }        

    path.push_str(format);
                            
    let _ = working_options.remove("format");                        

    //Check if its an rxcui encoded URL
    if validators::has_rxcui_path_parameter(function) {  
        let rxcui = working_options.get("rxcui")
                    .ok_or(RxNormError::MissingRxcui(function.to_string()))?;
        path = path.replace("{rxcui}",rxcui);
        let _ = working_options.remove("rxcui");
    } 

    //Assemble final URL with request
    let base = Url::parse(constants::RXNORM_DOMAIN)?;
    let mut url = base.join(&path)?;

    if !working_options.is_empty() {
        let mut query = url.query_pairs_mut();
        for (key,value) in working_options {
            //RxNorm api has a bug where the server will not accept RFC 3986 encoded strings
            query.append_pair(key,value);
        }
    }
    Ok(url)
}
