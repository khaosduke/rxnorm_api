use std::collections::HashMap;
use std::fmt::Write;

mod constants;
use constants::RXNORM_FUNCTIONS;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn build_get_request<'a>(function: &'a str, options:&'a HashMap<&'a str,&'a str>) -> String {
    
    let opt_str = build_options_string(options);
    println!("Got: {}",opt_str);
    return opt_str;
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
