
use std::{error::Error};


use chrono::{DateTime, Utc, Datelike};
use regex::Regex;

fn is_range( arg: &str ) -> bool {
    Regex::new( r"^\d+-\d+$" ).unwrap().is_match( arg )
}

fn is_day( arg: &str ) -> bool {
    Regex::new( r"(?i)Sunday|Monday|Tuesday|Wednesday|Thursday|Friday|Saturday" ).unwrap().is_match( arg )
}

#[derive(Debug)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

impl DayOfWeek {
    pub fn from_str( day: &str ) -> DayOfWeek {
        match day {
            "Sunday" => DayOfWeek::Sunday,
            "Monday" => DayOfWeek::Monday,
            "Tuesday" => DayOfWeek::Tuesday,
            "Wednesday" => DayOfWeek::Wednesday,
            "Thursday" => DayOfWeek::Thursday,
            "Friday" => DayOfWeek::Friday,
            "Saturday" => DayOfWeek::Saturday,
            _ => panic!( "Invalid day of week!" )
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DayOfWeek::Sunday => "Sunday".to_owned(),
            DayOfWeek::Monday => "Monday".to_owned(),
            DayOfWeek::Tuesday => "Tuesday".to_owned(),
            DayOfWeek::Wednesday => "Wednesday".to_owned(),
            DayOfWeek::Thursday => "Thursday".to_owned(),
            DayOfWeek::Friday => "Friday".to_owned(),
            DayOfWeek::Saturday => "Saturday".to_owned(),
        }
    }

    pub fn index(&self) -> u32 {
        match self {
            DayOfWeek::Sunday => 0,
            DayOfWeek::Monday => 1,
            DayOfWeek::Tuesday => 2,
            DayOfWeek::Wednesday => 3,
            DayOfWeek::Thursday => 4,
            DayOfWeek::Friday => 5,
            DayOfWeek::Saturday => 6,
        }
    }
}

#[derive(Debug)]
pub enum CronCommand {
    Asterisk,
    Number(u32),
    Range(u32, u32),
    DayOfWeek(DayOfWeek)
}

impl CronCommand {
    pub fn from_str( val: &str ) -> Result<CronCommand, Box<dyn Error>> {
        match val {
            day_str if is_day( day_str ) => {
                Ok(CronCommand::DayOfWeek( DayOfWeek::from_str( day_str ) ))
            },
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
            CronCommand::Range( min, max ) => format!( "{min} to {max}" ),
            CronCommand::DayOfWeek(d) => d.to_string(),
        }
    }

    pub fn is_valid( &self, current: u32, date: &DateTime<Utc> ) -> bool {
        match self {
            CronCommand::Asterisk => true,
            CronCommand::Number(n) => current == *n,
            CronCommand::Range( min, max ) => current <= *max && current >= *min,
            CronCommand::DayOfWeek(day) => date.weekday().num_days_from_sunday() == day.index(),
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
            CronCommand::DayOfWeek(_) => todo!(),
        }
    }
}