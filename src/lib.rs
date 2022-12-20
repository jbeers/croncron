
use std::{error::Error, fmt::Display};


use chrono::{prelude::*};

mod errors;
mod position;
mod command;

use position::{*};
use command::{*};
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

    macro_rules! matches_schedule {
        ($min:literal, $hour:literal, $day_of_month:literal, $month:literal, $day_of_week:literal, $date_str:literal ) => {
            assert!(
                CronSchedule::new( $min, $hour, $day_of_month, $month, $day_of_week ).unwrap().check_date( &$date_str.parse::<DateTime<Utc>>().unwrap() )
            )
        };
    }
    

    #[test]
    fn should_match_dates(){
        matches_schedule!( "*", "*", "*", "*", "*", "2022-12-17 11:36:00Z");
        matches_schedule!( "*", "*", "*", "*", "Monday", "2022-12-19 11:36:00Z");
    }

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

    #[test]
    fn should_fail_to_parse_range_invalid_hour_range(){
        let did_error = match CronSchedule::new( "*", "27-25", "*", "*", "7" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );

        let did_error = match CronSchedule::new( "*", "30-21", "*", "*", "7" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );

        let did_error = match CronSchedule::new( "*", "21-27", "*", "*", "7" ) {
            Err(_e) => true,
            _ => false
        };
        
        assert!( did_error );
    }

    #[test]
    fn should_increment_hour_range(){
        let c = CronSchedule::new( "*", "15-18", "*", "*", "*" ).unwrap();
        
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 11, 28, 1, 0, 0).unwrap() ),
            Utc.with_ymd_and_hms(2022, 11, 28, 15, 0, 0).unwrap()
        );

        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 11, 28, 17, 0, 0).unwrap() ),
            Utc.with_ymd_and_hms(2022, 11, 28, 18, 0, 0).unwrap()
        );

        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 11, 28, 19, 0, 0).unwrap() ),
            Utc.with_ymd_and_hms(2022, 11, 29, 15, 0, 0).unwrap()
        );
    }

    #[test]
    fn should_increment_month_range(){
        let c = CronSchedule::new( "*", "*", "*", "2-4", "*" ).unwrap();
        
        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 11, 28, 1, 0, 0).unwrap() ),
            Utc.with_ymd_and_hms(2023, 2, 1, 0, 0, 0).unwrap()
        );

        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 3, 2, 17, 0, 0).unwrap() ),
            Utc.with_ymd_and_hms(2022, 4, 1, 0, 0, 0).unwrap()
        );

        assert_eq!(
            c.get_next_occurrence( Utc.with_ymd_and_hms(2022, 4, 4, 19, 0, 0).unwrap() ),
            Utc.with_ymd_and_hms(2023, 2, 1, 0, 0, 0).unwrap()
        );
    }
}



macro_rules! validate_number {
    ($position:path where $n:ident between $min:literal and $max:literal) => {
        {
            #[allow(unused_comparisons)]
            if $n > $max || $n < $min {
                return Err( Box::new( errors::CronNumberParseError::new( &$position.to_string(), $min, $max ) ) );
            }
        }
    };
}

macro_rules! validate_range {
    ($position:path where $a:ident to $b:ident between $min:literal and $max:literal) => {
        {
            if $b > $max {
                return Err( Box::new( errors::CronNumberParseError::new( &$position.to_string(), $min, $max ) ) );
            }
            else if $a > $max {
                return Err( Box::new( errors::CronNumberParseError::new( &$position.to_string(), $min, $max ) ) );
            }
            else if $a >= $b {
                return Err( Box::new( errors::CronInvalidRange::new( &$position.to_string() ) ) );
            }
        }
    };
}



#[derive(Debug)]
struct CronArg{
    position: CronPosition,
    command: CronCommand
}

impl CronArg {
    fn parse( position: CronPosition, command_string: &str ) -> Result<CronArg, Box<dyn Error>>  {
        let command = CronCommand::from_str(command_string )?;

        match command {
            CronCommand::Number(n) => {
                match position {
                    CronPosition::Minute => validate_number!( CronPosition::Minute where n between 0 and 59 ),
                    CronPosition::Hour => validate_number!( CronPosition::Hour where n between 0 and 23 ),
                    CronPosition::DayOfMonth => validate_number!( CronPosition::DayOfMonth where n between 1 and 31 ),
                    CronPosition::Month => validate_number!( CronPosition::Month where n between 1 and 12 ),
                    CronPosition::DayOfWeek => validate_number!( CronPosition::DayOfWeek where n between 0 and 6 ),
                }
            },
            CronCommand::Range(min,max) => {
                match position {
                    CronPosition::Minute => validate_range!( CronPosition::Minute where min to max between 0 and 59 ),
                    CronPosition::Hour => validate_range!( CronPosition::Hour where min to max between 0 and 23 ),
                    CronPosition::DayOfMonth => validate_range!( CronPosition::DayOfMonth where min to max between 1 and 31 ),
                    CronPosition::Month => validate_range!( CronPosition::Month where min to max between 1 and 12 ),
                    CronPosition::DayOfWeek => validate_range!( CronPosition::DayOfWeek where min to max between 0 and 6 ),
                }
            },
            _ => {}
        }

        Ok( CronArg{ position, command } )
    }

    fn update_date( &self, date: &DateTime<Utc> ) -> DateTime<Utc> {
        let current_value = self.position.get_value_from_date( date );
        let next_value = self.command.get_next_value(current_value, self.position.get_min(), self.position.get_max() );
        
        // self.0.update_date(date, next_value)
        match self.command {
            CronCommand::Asterisk => date.clone(),
            _ => {
                self.position.update_date(date, next_value)
            }
        }
    }

    pub fn check_date( &self, date: &DateTime<Utc> ) -> bool {
        let current_value = self.position.get_value_from_date( date );
        self.command.is_valid(current_value, date)
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

    pub fn check_date( &self, date: &DateTime<Utc> ) -> bool {
        self.cron_day_of_week.check_date( date )
        && self.cron_month.check_date( &date )
        && self.cron_day_of_month.check_date( &date )
        && self.cron_hour.check_date( &date )
        && self.cron_minute.check_date( &date )
    }

    pub fn to_string( &self ) -> String {
        format!(
            "{} {} {} {} {}",
            self.cron_minute.command.to_string(),
            self.cron_hour.command.to_string(),
            self.cron_day_of_month.command.to_string(),
            self.cron_month.command.to_string(),
            self.cron_day_of_week.command.to_string(),
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

