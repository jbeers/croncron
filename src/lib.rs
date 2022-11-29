
use std::{num::ParseIntError, error::Error, ops::Add};

use chrono::prelude::*;
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
        let _c = CronCommand::from_str( "15 * * * *");

        assert!( true );
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
}

pub enum CronCommand {
    Asterix,
    Number(u32),
}

impl CronCommand {
    fn from_str( val: &str ) -> Result<CronCommand, Box<dyn Error>> {
        match val {
            "*" => Ok(CronCommand::Asterix),
            v => {
                let num = u32::from_str_radix( v, 10 )?;

                Ok(CronCommand::Number( num ))
            }
        }
    }

    fn to_string( &self ) -> String {
        match self {
            CronCommand::Asterix => "*".to_owned(),
            CronCommand::Number(n) => n.to_string() 
        }
    }
}

enum CronPosition {
    Minute,
    Hour,
    DayOfMonth,
    Month,
    DayOfWeek
}

struct CronArg( CronPosition, CronCommand );

impl CronArg {
    fn update_date( &self, date: &DateTime<Utc> ) -> DateTime<Utc> {
        match self.1 {
            CronCommand::Asterix => date.clone(),
            CronCommand::Number(n) => {
                match self.0 {
                    CronPosition::Minute => {
                        let next = date.clone().with_minute( n ).unwrap();

                        if next.lt( date ) {
                            next.with_hour( date.hour() + 1 ).unwrap()
                        }
                        else {
                            next
                        }
                    },
                    _ => date.clone()
                }
            }
        }
    }
}

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
            cron_minute: CronArg( CronPosition::Minute, CronCommand::from_str( minute )? ),
            cron_hour: CronArg( CronPosition::Hour, CronCommand::from_str( hour )? ),
            cron_day_of_month: CronArg( CronPosition::DayOfMonth, CronCommand::from_str( day_of_month )? ),
            cron_month: CronArg( CronPosition::Month, CronCommand::from_str( month )? ),
            cron_day_of_week: CronArg( CronPosition::DayOfWeek, CronCommand::from_str( day_of_week )? ),
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
        let parts: Vec<&str> = cron_string.split( " " ).collect();

        if parts.len() != 5 {
            return Err( format!( "Invalid Cron string {}", cron_string ) );
        }

        
        match CronSchedule::new("*", "*", "*", "*", "*" ) {
            Ok( c ) => Ok( c ),
            _ => Err( format!( "Invalid Cron string {}", cron_string ) )
        }
    }

    pub fn get_next_occurrence( &self, start: DateTime<Utc> ) -> DateTime<Utc> {
        self.cron_minute.update_date( &start )
    }
}

