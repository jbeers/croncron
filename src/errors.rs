use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CronNumberParseError {
    details: String
}

impl CronNumberParseError {
    pub fn new( position: &str, min: u32, max: u32 ) -> CronNumberParseError {
        CronNumberParseError { details: format!("{} must be between {} and {} inclusive", position, min, max ) }
    }
}
impl Display for CronNumberParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.details )
    }
}

impl Error for CronNumberParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct CronInvalidRange {
    details: String
}
const DETAILS: &str = "Invalid range: the range min must be less than the range max";

impl CronInvalidRange {
    pub fn new( position: &str ) -> CronNumberParseError {
        CronNumberParseError { details: position.clone().into() }
    }
}

impl Display for CronInvalidRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {DETAILS}", self.details )
    }
}

impl Error for CronInvalidRange {
    fn description(&self) -> &str {
        DETAILS
    }
}

#[derive(Debug)]
pub struct CronInvalidArgument {
    details: String
}

impl CronInvalidArgument {
    pub fn new( position: &str, arg: &str ) -> CronNumberParseError {
        CronNumberParseError { details: format!("The argument {} is invalid in the {} position", arg, position ) }
    }
}
impl Display for CronInvalidArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.details )
    }
}

impl Error for CronInvalidArgument {
    fn description(&self) -> &str {
        &self.details
    }
}