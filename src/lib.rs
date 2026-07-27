use std::collections::HashMap;
use std::fmt::Write;

mod constants;

mod error;
pub use error::RxNormError;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn build_get_request<'a>(function: &'a str, 
                             options:&'a HashMap<&'a str,&'a str>)
        -> Result<String,RxNormError> {
    
    //Don't even continue if function or options are invalid
    if !verify_function_name(function)? ||
       !verify_options_hash(function,options)? {
       return Err(RxNormError::InvalidFunctionOrOption);
    }
    
    
    //Since we might change options, and they arent that big we can copy to the heap
    let mut working_options = options.clone();


    //Get the standard functions hashmap of options
    let (std_rest_str,std_opts_hash) = 
        constants::RXNORM_FUNCTIONS.get(function)
                .ok_or(RxNormError::GenericError)?;

    let mut final_route_str;    
    if working_options.contains_key("format") {
        let format = working_options.get("format")
                    .ok_or(RxNormError::UnWrapError("format".to_string()))?;
        final_route_str = format!("{}{}",std_rest_str.to_string(),format);
    } else {
        final_route_str = format!("{}.xml",std_rest_str.to_string());
    }
    let _ = working_options.remove("format");


    //Check if its an rxcui encoded URL
    if std_opts_hash.contains_key("rxcui") {
        let rxcui = working_options.get("rxcui")
                    .ok_or(RxNormError::RXCUIExpected(function.to_string()))?;

        final_route_str = final_route_str.replace("{rxcui}",rxcui);
        let _ = working_options.remove("rxcui");
    } 

    let rest_str;
    if !working_options.is_empty() {
        let opt_str = build_options_string(&working_options);
        rest_str = format!("{}{}?{}",constants::RXNORM_DOMAIN,final_route_str,opt_str);
    } else {
        rest_str = format!("{}{}",constants::RXNORM_DOMAIN,final_route_str);
    }    

    println!("Got: {}",rest_str);
    Ok(rest_str)
}

fn build_options_string<'a>(options:&'a HashMap<&'a str,&'a str>) -> String {
    let mut buffer = String::new();

    for (key,value) in options {
        let _ = write!(buffer,"{}={}&",key,value);
    }
    //Remove the last &
    buffer.pop();
    return buffer;
}

fn verify_function_name<'a>(function: &'a str) 
    -> Result<bool, RxNormError> {
    if !constants::RXNORM_FUNCTIONS.contains_key(function) {
        return Err(RxNormError::InvalidFunction(function.to_string()));
    }
    Ok(true)
}

fn verify_options_hash<'a>(function: &'a str, 
                           options:&'a HashMap<&'a str,&'a str>) 
    -> Result<bool,RxNormError> {
    //Get the function's options hash
    let (_,std_opt_hash) = constants::RXNORM_FUNCTIONS
                        .get(function)
                        .ok_or(RxNormError::UnWrapError(function.to_string()))?;
    
    //Check that the options in the supplied hash exist in the functions standard opts hash  
    for (parameter,_) in options {
        //Ignore format and rxcui
        if *parameter == constants::FORMAT_PARAMETER ||
           *parameter == constants::RXCUI_PARAMETER {
            continue;
           }
        //Error out if there is an invalid option parameter
        if !std_opt_hash.contains_key(parameter) {
            return Err(RxNormError::InvalidOptions(parameter.to_string()));
        }   
    }                    
    Ok(true) //Options are valid
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
