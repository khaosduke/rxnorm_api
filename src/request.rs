use std::collections::HashMap;
use url::Url;

//mod constants;
//use constants::RXNORM_FUNCTIONS;

//mod error;
//pub use error::RxNormError;

use crate::constants;
use crate::constants::RXNORM_FUNCTIONS;
use crate::error::RxNormError;

pub fn build_get_request(function: &str, 
                             options:&HashMap<&str,&str>)
        -> Result<Url,RxNormError> {
    
    //Don't even continue if function or options are invalid
    verify_function_name(function)?; 
    verify_options_hash(function,options)?; 
    
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
    if has_rxcui_path_parameter(function) {  
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

//VALIDATION FUNCTIONS
fn verify_function_name(function: &str) 
    -> Result<(), RxNormError> {
    if !constants::RXNORM_FUNCTIONS.contains_key(function) {
        return Err(RxNormError::InvalidFunction(function.to_string()));
    }
    Ok(())
}

fn verify_options_hash(function: &str, 
                           options:&HashMap<&str,&str>) 
    -> Result<(),RxNormError> {
    //Get the function's options hash
    let (_,std_opt_hash) = constants::RXNORM_FUNCTIONS
                        .get(function)
                        .ok_or(RxNormError::UnWrapError(function.to_string()))?;
    
    //Save the result, does this function require RXCUI:
    let is_rxcui = has_rxcui_path_parameter(function);
    if is_rxcui &&
       !options.contains_key(constants::RXCUI_PARAMETER) {
        return Err(RxNormError::MissingRxcui(function.to_string()))
       }
    
    //Check that the options in the supplied hash exist in the functions standard opts hash  
    for (parameter,_) in options {
        //Ignore format
        if *parameter == constants::FORMAT_PARAMETER {
            continue;
        }
        //Error out if there is an invalid option parameter
        if !std_opt_hash.contains_key(parameter) {
            return Err(RxNormError::InvalidOptions(parameter.to_string()));
        }   
    }                    
    Ok(()) //Options are valid
}

fn has_rxcui_path_parameter(function: &str) -> bool {
    //NOTE this should be safe because this is never run out of context of this lib
    let (_,std_opt_hash) = constants::RXNORM_FUNCTIONS
                        .get(function)
                        .unwrap(); 
     return std_opt_hash.contains_key("rxcui");                   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}