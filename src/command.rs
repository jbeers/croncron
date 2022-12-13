
use std::{error::Error};


use regex::Regex;

fn is_range( arg: &str ) -> bool {
    Regex::new( r"^\d+-\d+$" ).unwrap().is_match( arg )
}

#[derive(Debug)]
pub enum CronCommand {
    Asterisk,
    Number(u32),
    Range(u32, u32)
}

impl CronCommand {
    pub fn from_str( val: &str ) -> Result<CronCommand, Box<dyn Error>> {
        match val {
            range_str if is_range( range_str ) => {
                let parts : Vec<&str> = range_str.split( '-' ).collect();

                let min = u32::from_str_radix( parts[0], 10 )?;
                let max = u32::from_str_radix( parts[1], 10 )?;

                Ok(CronCommand::Range(min, max))
            },
            "*" => Ok(CronCommand::Asterisk),
            v => {
                let num = u32::from_str_radix( v, 10 )?;

                Ok(CronCommand::Number( num ))
            }
        }
    }

    pub fn to_string( &self ) -> String {
        match self {
            CronCommand::Asterisk => "*".to_owned(),
            CronCommand::Number(n) => n.to_string() ,
            CronCommand::Range( min, max ) => format!( "{min} to {max}" )
        }
    }

    pub fn get_next_value( &self, current: u32, min: u32, max: u32 ) -> u32 {
        match self {
            CronCommand::Asterisk => {
                if current == max {
                    min
                }
                else {
                    current + 1
                }
            },
            CronCommand::Number(num) => *num,
            CronCommand::Range(min, max) => {
                if current >= *max || current < *min {
                    *min
                }
                else {
                    current + 1
                }
            },
        }
    }
}