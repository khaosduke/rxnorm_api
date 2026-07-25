use std::collections::HashMap;
use std::fmt::Write;

mod constants;
use constants::RXNORM_FUNCTIONS;
use constants::RXNORM_DOMAIN;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn build_get_request<'a>(function: &'a str, options:&'a HashMap<&'a str,&'a str>) -> Result<String,&'static str> {
    //Since we might change options, and they arent that big we can copy to the heap
    let mut working_options = options.clone();


    //Get the standard functions hashmap of options
    let (std_rest_str,std_opts_hash) = 
        constants::RXNORM_FUNCTIONS.get(function).ok_or("RX Norm functions inaccesible")?;

    let mut final_route_str;    
    if working_options.contains_key("format") {
        let format = working_options.get("format").ok_or("Can't get format from hash")?;
        final_route_str = format!("{}{}",std_rest_str.to_string(),format);
    } else {
        final_route_str = format!("{}.xml",std_rest_str.to_string());
    }
    let _ = working_options.remove("format");


    //Check if its an rxcui encoded URL
    if std_opts_hash.contains_key("rxcui") {
        let rxcui = working_options.get("rxcui").ok_or("RXCUI expected in options, none found")?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
