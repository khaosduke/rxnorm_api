use std::fmt;

#[derive(Debug)]
pub enum RxNormError {
    InvalidFunction(String),
    InvalidOptions(String),
    RequestFailed(String),
    InvalidResponse(String),
    InvalidFunctionOrOption,
    UnWrapError(String),
    MissingRxcui(String),
    Url(url::ParseError),
    InvalidFormat(String),
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
            RxNormError::MissingRxcui(function) => {
                write!(f,"{function} is a function which required RXCUI, None provided")    
            }
            RxNormError::Url(error) => {
                write!(f, "Unable to build RxNorm URL: {error}")
            }
            RxNormError::InvalidFormat(format) => {
                write!(f,"Invalid format: {format}")
            }

        }
    }
}
impl From<url::ParseError> for RxNormError {
    fn from(error: url::ParseError) -> Self {
        Self::Url(error)
    }
}
impl std::error::Error for RxNormError {}