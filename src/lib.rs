
use std::{error::Error, fmt::Display};
use regex::Regex;

use chrono::{prelude::*};

mod errors;
/*
https://en.wikipedia.org/wiki/Cron
# ┌───────────── minute (0 - 59)
# │ ┌───────────── hour (0 - 23)
# │ │ ┌───────────── day of the month (1 - 31)
# │ │ │ ┌───────────── month (1 - 12)
# │ │ │ │ ┌───────────── day of the week (0 - 6) (Sunday to Saturday;
# │ │ │ │ │                                   7 is also Sunday on some systems)
# │ │ │ │ │
# │ │ │ │ │
# * * * * * <command to execute>
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_str_should_produce_cron_string(){
        let c = CronSchedule::new( "15", "*", "*", "*", "*" ).unwrap();

        assert_eq!( c.to_string(), "15 * * * *" );
    }

    #[test]
    fn from_str_should_return_cronschedule(){
        let c = CronSchedule::from_str( "15 * * * *" ).unwrap();

        assert_eq!( c.to_string(), "15 * * * *" );
    }

    #[test]
    fn should_return_next_occurrence(){
        let c = CronSchedule::new( "15", "*", "*", "*", "*" ).unwrap();

        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2020, 11, 28, 5, 0, 0).unwrap() ),
            Utc.with_ymd_and_hms(2020, 11, 28, 5, 15, 0).unwrap()
        );
    }

    #[test]
    fn should_increment_hour_if_minute_in_past(){
        let c = CronSchedule::new( "15", "*", "*", "*", "*" ).unwrap();

        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2020, 11, 28, 5, 20, 0).unwrap() ),
            Utc.with_ymd_and_hms(2020, 11, 28, 6, 15, 0).unwrap()
        );
    }

    #[test]
    fn should_support_numbers_for_hour(){
        let c = CronSchedule::new( "*", "7", "*", "*", "*" ).unwrap();
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2020, 11, 28, 5, 20, 0).unwrap() ),
            Utc.with_ymd_and_hms(2020, 11, 28, 7, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_support_numbers_for_day_of_month(){
        let c = CronSchedule::new( "*", "*", "30", "*", "*" ).unwrap();
        
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2020, 11, 28, 1, 20, 0).unwrap() ),
            Utc.with_ymd_and_hms(2020, 11, 30, 0, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_support_numbers_for_month(){
        let c = CronSchedule::new( "*", "*", "*", "1", "*" ).unwrap();
        
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2020, 12, 28, 1, 20, 0).unwrap() ),
            Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_support_numbers_for_day_of_week(){
        let c = CronSchedule::new( "*", "*", "*", "*", "3" ).unwrap();
        
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 11, 28, 1, 20, 0).unwrap() ),
            Utc.with_ymd_and_hms(2022, 11, 30, 0, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_error_on_incorrect_minute_value(){
        let did_error = match CronSchedule::new( "60", "*", "*", "*", "3" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );
    }

    #[test]
    fn should_error_on_incorrect_hour_value(){
        let did_error = match CronSchedule::new( "*", "24", "*", "*", "3" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );
    }

    #[test]
    fn should_error_on_incorrect_day_of_month_value(){
        let did_error = match CronSchedule::new( "*", "*", "32", "*", "3" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );
    }

    #[test]
    fn should_error_on_incorrect_month_value(){
        let did_error = match CronSchedule::new( "*", "*", "*", "0", "3" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );
    }

    #[test]
    fn should_error_on_incorrect_day_of_week_value(){
        let did_error = match CronSchedule::new( "*", "*", "*", "*", "7" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );
    }

    #[test]
    fn should_parse_range_minute_value(){
        let c = CronSchedule::new( "5-10", "*", "*", "*", "*" ).unwrap();
        
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 11, 28, 1, 5, 0).unwrap() ),
            Utc.with_ymd_and_hms(2022, 11, 28, 1, 6, 0).unwrap()
        );
    }

    #[test]
    fn should_increment_minute_to_min_range(){
        let c = CronSchedule::new( "5-10", "*", "*", "*", "*" ).unwrap();
        
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 11, 28, 1, 1, 0).unwrap() ),
            Utc.with_ymd_and_hms(2022, 11, 28, 1, 5, 0).unwrap()
        );
    }

    #[test]
    fn should_increment_minute_to_min_range_if_above_max(){
        let c = CronSchedule::new( "5-10", "*", "*", "*", "*" ).unwrap();
        
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 11, 28, 1, 11, 0).unwrap() ),
            Utc.with_ymd_and_hms(2022, 11, 28, 2, 5, 0).unwrap()
        );
    }

    #[test]
    fn should_increment_minute_to_next_within_range(){
        let c = CronSchedule::new( "5-10", "*", "*", "*", "*" ).unwrap();
        
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 11, 28, 1, 6, 0).unwrap() ),
            Utc.with_ymd_and_hms(2022, 11, 28, 1, 7, 0).unwrap()
        );
    }

    #[test]
    fn should_fail_to_parse_range_invalid_min(){
        let did_error = match CronSchedule::new( "60-10", "*", "*", "*", "7" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );
    }

    #[test]
    fn should_fail_to_parse_range_invalid_max(){
        let did_error = match CronSchedule::new( "50-75", "*", "*", "*", "7" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );
    }

    #[test]
    fn should_fail_to_parse_range_invalid_range(){
        let did_error = match CronSchedule::new( "48-32", "*", "*", "*", "7" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );
    }
}

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
    fn from_str( val: &str ) -> Result<CronCommand, Box<dyn Error>> {
        match val {
            r if is_range( val ) => {
                let parts : Vec<&str> = val.split( '-' ).collect();

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

    fn to_string( &self ) -> String {
        match self {
            CronCommand::Asterisk => "*".to_owned(),
            CronCommand::Number(n) => n.to_string() ,
            CronCommand::Range( min, max ) => format!( "{min} to {max}" )
        }
    }
}

#[derive(Debug)]
enum CronPosition {
    Minute,
    Hour,
    DayOfMonth,
    Month,
    DayOfWeek
}

impl Display for CronPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            CronPosition::Minute => "Minute",
            CronPosition::Hour => "Hour",
            CronPosition::DayOfMonth => "DayOfMonth",
            CronPosition::Month => "Month",
            CronPosition::DayOfWeek => "DayOfWeek",
        };

        write!( f, "{display}" )
    }
}



#[derive(Debug)]
struct CronArg( CronPosition, CronCommand );

impl CronArg {
    fn parse( position: CronPosition, command_string: &str ) -> Result<CronArg, Box<dyn Error>>  {
        let command = CronCommand::from_str(command_string )?;

        match command {
            CronCommand::Number(n) => {
                match position {
                    CronPosition::Minute => {
                        if n > 59 {
                            return Err( Box::new( errors::CronNumberParseError::new( &CronPosition::Minute.to_string(), 0, 59 ) ) );
                        }
                    },
                    CronPosition::Hour => {
                        if n > 23 {
                            return Err( Box::new( errors::CronNumberParseError::new( &CronPosition::Hour.to_string(), 0, 23 ) ) );
                        }
                    },
                    CronPosition::DayOfMonth => {
                        if n > 31 || n == 0 {
                            return Err( Box::new( errors::CronNumberParseError::new( &CronPosition::DayOfMonth.to_string(), 1, 31 ) ) );
                        }
                    },
                    CronPosition::Month => {
                        if n > 12 || n == 0 {
                            return Err( Box::new( errors::CronNumberParseError::new( &CronPosition::Month.to_string(), 1, 12 ) ) );
                        }
                    },
                    CronPosition::DayOfWeek => {
                        if n > 6 {
                            return Err( Box::new( errors::CronNumberParseError::new( &CronPosition::DayOfWeek.to_string(), 0, 6 ) ) );
                        }
                    }
                }
            },
            CronCommand::Range(min,max) => {
                match position {
                    CronPosition::Minute => {
                        if max > 59 {
                            return Err( Box::new( errors::CronNumberParseError::new( &CronPosition::Minute.to_string(), 0, 59 ) ) );
                        }
                        else if min > 59 {
                            return Err( Box::new( errors::CronNumberParseError::new( &CronPosition::Minute.to_string(), 0, 59 ) ) );
                        }
                        else if min >= max {
                            return Err( Box::new( errors::CronInvalidRange::new( &CronPosition::Minute.to_string() ) ) );
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        Ok( CronArg( position, command ) )
    }

    fn update_date( &self, date: &DateTime<Utc> ) -> DateTime<Utc> {
        match self.1 {
            CronCommand::Asterisk => date.clone(),
            CronCommand::Range(min, max) => {
                match self.0 {
                    CronPosition::Minute => {
                        let next = date.clone();
                        let current_minute = next.minute();

                        if max <= current_minute {
                            next.with_hour( date.hour() + 1 ).unwrap().with_minute(min).unwrap()
                        }
                        else if current_minute < min {
                            next.with_minute(min).unwrap()
                        }
                        else {
                            next.with_minute(current_minute + 1).unwrap()
                        }
                    },
                    _ => date.clone()
                }
            },
            CronCommand::Number(n) => {
                match self.0 {
                    CronPosition::DayOfWeek => {
                        let current_weekday = date.weekday().num_days_from_sunday();
                        let to_add = if current_weekday < n {
                            n - current_weekday
                        }
                        else {
                            ( current_weekday + 7 ) - n
                        };


                        let days = chrono::Days::new( to_add.into() );

                        let next = date.checked_add_days( days ).unwrap()
                            .with_hour( 0 ).unwrap()
                            .with_minute(0).unwrap();

                        next

                    },
                    CronPosition::Month => {
                        let next = date.clone()
                            .with_month( n ).unwrap()
                            .with_day(1).unwrap()
                            .with_hour( 0 ).unwrap()
                            .with_minute(0).unwrap();

                        if next.lt( date ) {
                            next.with_year( date.year() + 1 ).unwrap()
                        }
                        else {
                            next
                        }

                    },
                    CronPosition::DayOfMonth => {
                        let next = date.clone().with_day(n).unwrap().with_hour( 0 ).unwrap().with_minute(0).unwrap();

                        if next.lt( date ) {
                            next.with_month( date.month() + 1 ).unwrap()
                        }
                        else {
                            next
                        }

                    },
                    CronPosition::Hour => {
                        let next = date.clone().with_hour( n ).unwrap().with_minute(0).unwrap();

                        if next.lt( date ) {
                            next.with_day( date.day() + 1 ).unwrap()
                        }
                        else {
                            next
                        }

                    },
                    CronPosition::Minute => {
                        let next = date.clone().with_minute( n ).unwrap();

                        if next.lt( date ) {
                            next.with_hour( date.hour() + 1 ).unwrap()
                        }
                        else {
                            next
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct CronSchedule {
    cron_minute: CronArg,
    cron_hour: CronArg,
    cron_day_of_month: CronArg,
    cron_month: CronArg,
    cron_day_of_week: CronArg
}

impl CronSchedule {

    pub fn new( minute: &str, hour: &str, day_of_month: &str, month: &str, day_of_week: &str ) -> Result<CronSchedule, Box<dyn Error>> {
        Ok(CronSchedule {
            cron_minute: CronArg::parse( CronPosition::Minute, minute )?,
            cron_hour: CronArg::parse( CronPosition::Hour, hour )?,
            cron_day_of_month: CronArg::parse( CronPosition::DayOfMonth, day_of_month )?,
            cron_month: CronArg::parse( CronPosition::Month, month )?,
            cron_day_of_week: CronArg::parse( CronPosition::DayOfWeek, day_of_week )?,
        })
    }

    pub fn to_string( &self ) -> String {
        format!(
            "{} {} {} {} {}",
            self.cron_minute.1.to_string(),
            self.cron_hour.1.to_string(),
            self.cron_day_of_month.1.to_string(),
            self.cron_month.1.to_string(),
            self.cron_day_of_week.1.to_string(),
        )
    }

    pub fn from_str( cron_string: &str ) -> Result<CronSchedule, String> {
        let parts: Vec<&str> = cron_string.split( ' ' ).collect();

        if parts.len() != 5 {
            return Err( format!( "Invalid Cron string {}", cron_string ) );
        }

        match CronSchedule::new( parts[ 0 ], parts[ 1 ], parts[ 2 ], parts[ 3 ], parts[ 4 ] ) {
            Ok( c ) => Ok( c ),
            _ => Err( format!( "Invalid Cron string {}", cron_string ) )
        }
    }

    pub fn get_next_occurrence( &self, start: DateTime<Utc> ) -> DateTime<Utc> {
        let mut date = self.cron_day_of_week.update_date( &start );
        date = self.cron_month.update_date( &date );
        date = self.cron_day_of_month.update_date( &date );
        date = self.cron_hour.update_date( &date );
        self.cron_minute.update_date( &date )
    }
}

