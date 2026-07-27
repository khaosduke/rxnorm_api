use crate::constants;
use crate::errors::RxNormError;

use std::collections::HashMap;


//VALIDATION FUNCTIONS
pub fn verify_function_name(function: &str) 
    -> Result<(), RxNormError> {
    if !constants::RXNORM_FUNCTIONS.contains_key(function) {
        return Err(RxNormError::InvalidFunction(function.to_string()));
    }
    Ok(())
}

pub fn verify_options_hash(function: &str, 
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

pub fn has_rxcui_path_parameter(function: &str) -> bool {
    //NOTE this should be safe because this is never run out of context of this lib
    let (_,std_opt_hash) = constants::RXNORM_FUNCTIONS
                        .get(function)
                        .unwrap(); 
     return std_opt_hash.contains_key("rxcui");                   
}