use std::fmt;

#[derive(Debug)]
pub enum RxNormError {
    InvalidFunction(String),
    InvalidOptions(String),
    RequestFailed(String),
    InvalidResponse(String),
    InvalidFunctionOrOption,
    UnWrapError(String),
    RXCUIExpected(String),
    GenericError
}

impl fmt::Display for RxNormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RxNormError::InvalidFunction(name) => {
                write!(f, "Unknown RxNorm function: {name}")
            }

            RxNormError::InvalidOptions(message) => {
                write!(f, "Invalid RxNorm options: {message}")
            }

            RxNormError::RequestFailed(message) => {
                write!(f, "RxNorm request failed: {message}")
            }

            RxNormError::InvalidResponse(message) => {
                write!(f, "Invalid RxNorm response: {message}")
            }
            RxNormError::InvalidFunctionOrOption => {
                write!(f,"Invalid Function or Option, see above")
            }
            RxNormError::UnWrapError(value) => {
                write!(f,"Unable to unwrap value: {value}")
            }
            RxNormError::GenericError => {
                write!(f,"Dont know what happened")
            }
            RxNormError::RXCUIExpected(function) => {
                write!(f,"{function} is a function which required RXCUI, None provided")    
            }
        }
    }
}

impl std::error::Error for RxNormError {}