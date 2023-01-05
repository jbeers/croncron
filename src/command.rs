
use std::{error::Error};


use chrono::{DateTime, Utc, Datelike};
use regex::Regex;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn w_should_match_weekday(){
        let command = CronCommand::W( 5 );
        let date = "2022-12-5 11:36:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!( command.is_valid( 5, &date ) );
    }

    #[test]
    fn w_should_not_match_sunday(){
        let command = CronCommand::W( 4 );
        let date = "2022-12-4 11:36:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!( !command.is_valid( 4, &date ) );
    }

    #[test]
    fn w_should_match_next_monday(){
        let command = CronCommand::W( 4 );
        let date = "2022-12-5 11:36:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!( command.is_valid( 5, &date ) );
    }

    #[test]
    fn w_should_not_match_saturday(){
        let command = CronCommand::W( 3 );
        let date = "2022-12-3 11:36:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!( !command.is_valid( 3, &date ) );
    }

    #[test]
    fn w_should_match_prev_friday(){
        let command = CronCommand::W( 3 );
        let date = "2022-12-2 11:36:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!( command.is_valid( 2, &date ) );
    }

    #[test]
    fn w_should_match_next_monday_if_early_month(){
        let command = CronCommand::W( 1 );
        let date = "2022-10-3 11:36:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!( command.is_valid( 3, &date ) );
    }

    #[test]
    fn l_should_match_last_friday(){
        let command = CronCommand::L( 5 );
        let date = "2023-01-27 11:36:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!( command.is_valid( 5, &date ) );
    }

    #[test]
    fn l_should_not_match_first_friday(){
        let command = CronCommand::L( 5 );
        let date = "2023-01-6 11:36:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!( !command.is_valid( 5, &date ) );
    }
}


fn is_range( arg: &str ) -> bool {
    Regex::new( r"^\d+-\d+$" ).unwrap().is_match( arg )
}

fn is_day( arg: &str ) -> bool {
    Regex::new( r"(?i)Sunday|Monday|Tuesday|Wednesday|Thursday|Friday|Saturday" ).unwrap().is_match( arg )
}

fn is_interval( arg: &str ) -> bool {
    Regex::new( r"/\d+" ).unwrap().is_match( arg )
}

fn is_w( arg: &str ) -> bool {
    Regex::new( r"\d+W" ).unwrap().is_match( arg )
}

fn is_l( arg: &str ) -> bool {
    Regex::new( r"\d+L" ).unwrap().is_match( arg )
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
    DayOfWeek(DayOfWeek),
    Interval(u32),
    W(u32),
    L(u32)
}

impl CronCommand {
    pub fn from_str( val: &str ) -> Result<CronCommand, Box<dyn Error>> {
        match val {
            l_str if is_l( l_str ) => {
                let num: u32 = l_str.replace(r"L", "" ).parse()?;

                Ok(CronCommand::L( num ))
            },
            w_str if is_w( w_str ) => {
                let num: u32 = w_str.replace(r"W", "" ).parse()?;

                Ok(CronCommand::W( num ))
            },
            interval_str if is_interval( interval_str ) => {
                let num: u32 = interval_str.replace("/", "" ).parse()?;
                
                Ok(CronCommand::Interval( num ))
            },
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
            CronCommand::Interval(i) => format!( "/{i}" ),
            CronCommand::W(n) => format!( "{n}W" ),
            CronCommand::L(n) => format!( "{n}L" ),
        }
    }

    pub fn is_valid( &self, current: u32, date: &DateTime<Utc> ) -> bool {
        match self {
            CronCommand::Asterisk => true,
            CronCommand::Number(n) => current == *n,
            CronCommand::Range( min, max ) => current <= *max && current >= *min,
            CronCommand::DayOfWeek(day) => date.weekday().num_days_from_sunday() == day.index(),
            CronCommand::Interval(i) => (current % *i )== 0,
            CronCommand::W(n) => {
                if current == *n && date.weekday().num_days_from_monday() < 5 {
                    true
                }
                else if date.weekday().num_days_from_sunday() == 1 && ( date.day() - 1 ) == *n {
                    true 
                }
                else if date.weekday().num_days_from_sunday() == 5 && ( date.day() + 1 ) == *n {
                    true 
                }
                else if *n == 1 && date.day() == 3 {
                    true 
                }
                else {
                    false
                }
            },
            CronCommand::L(n) => {
                let next_week = date.checked_add_days(chrono::Days::new(7) ).unwrap();
                if current == *n && next_week.month() > date.month() {
                    true
                }
                else {
                    false
                }
            }
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
            CronCommand::Interval(i) => {
                if *i > current {
                    return *i;
                }

                let m = current / i;
                let next = if m == 0 {
                    current + m
                }
                else {
                    current + i
                };

                if next < max {
                    next
                }
                else {
                    *i
                }
            },
            CronCommand::W(n) => todo!(),
            CronCommand::L(n) => todo!()
        }
    }
}