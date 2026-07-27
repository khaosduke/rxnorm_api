use std::collections::HashMap;
use url::Url;

mod constants;
use constants::RXNORM_FUNCTIONS;

mod error;
pub use error::RxNormError;



pub fn build_get_request<'a>(function: &'a str, 
                             options:&'a HashMap<&'a str,&'a str>)
        -> Result<Url,Box<dyn std::error::Error>> {
    
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
        
    //Get the format, 
    //Normalize, replace any "." in case its put in the format as ".json" or ".xml"
    //Add "." in its proper place
    path.push('.');
    let format = working_options.get("format")
                    .copied()
                    .unwrap_or("xml");
                    
    path.push_str(format);
                            
    let _ = working_options.remove("format");                        

    //Check if its an rxcui encoded URL
    if is_rxcui_function(function) {  
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
            query.append_pair(key,value);
        }
    }
    Ok(url)
}

//VALIDATION FUNCTIONS
fn verify_function_name<'a>(function: &'a str) 
    -> Result<(), RxNormError> {
    if !constants::RXNORM_FUNCTIONS.contains_key(function) {
        return Err(RxNormError::InvalidFunction(function.to_string()));
    }
    Ok(())
}

fn verify_options_hash<'a>(function: &'a str, 
                           options:&'a HashMap<&'a str,&'a str>) 
    -> Result<(),RxNormError> {
    //Get the function's options hash
    let (_,std_opt_hash) = constants::RXNORM_FUNCTIONS
                        .get(function)
                        .ok_or(RxNormError::UnWrapError(function.to_string()))?;
    
    //Save the result, does this function require RXCUI:
    let is_rxcui = is_rxcui_function(function);
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

fn is_rxcui_function<'a>(function: &'a str) -> bool {
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
