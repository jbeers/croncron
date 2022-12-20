use std::fmt::Display;

use chrono::{DateTime, Utc, TimeZone, Timelike, Datelike, Date};


#[derive(Debug)]
pub enum CronPosition {
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

impl CronPosition {
    pub fn update_date( &self, date: &DateTime<Utc>, next_value: u32 ) -> DateTime<Utc> {
        match self {
            CronPosition::Minute => {
                let next = date.clone();
                let current_minute = next.minute();

                if current_minute >= next_value {
                    return next.with_hour( date.hour() + 1 ).unwrap().with_minute(next_value).unwrap()
                }

                next.with_minute(next_value).unwrap()
            },
            CronPosition::Hour => {
                let next = date.clone();
                let current_hour = next.hour();

                if current_hour >= next_value {
                    return next.with_day( date.day() + 1 ).unwrap().with_hour( next_value ).unwrap().with_minute(0).unwrap();
                }

                next.with_hour( next_value ).unwrap().with_minute(0).unwrap()
            },
            CronPosition::Month => {
                let next = date.clone();
                let current_month = next.month();

                if current_month >= next_value {
                    return next.with_year( date.year() + 1 ).unwrap()
                        .with_month( next_value ).unwrap()
                        .with_day( 1 ).unwrap()
                        .with_hour( 0 ).unwrap()
                        .with_minute(0).unwrap();
                }

                next.with_month( next_value ).unwrap()
                        .with_day( 1 ).unwrap()
                        .with_hour( 0 ).unwrap()
                        .with_minute(0).unwrap()
            },
            CronPosition::DayOfMonth => {
                let next = date.clone().with_day(next_value).unwrap().with_hour( 0 ).unwrap().with_minute(0).unwrap();

                if next.lt( date ) {
                    next.with_month( date.month() + 1 ).unwrap()
                }
                else {
                    next
                }
            }
            CronPosition::DayOfWeek => {
                let current_weekday = date.weekday().num_days_from_sunday();
                let to_add = if current_weekday < next_value {
                    next_value - current_weekday
                }
                else {
                    ( current_weekday + 7 ) - next_value
                };


                let days = chrono::Days::new( to_add.into() );

                let next = date.checked_add_days( days ).unwrap()
                    .with_hour( 0 ).unwrap()
                    .with_minute(0).unwrap();

                next
            }
        }
    }

    pub fn get_min( &self ) -> u32 {
        match self {
            CronPosition::Minute => 0,
            CronPosition::Hour => 0,
            CronPosition::DayOfMonth => 1,
            CronPosition::Month => 1,
            CronPosition::DayOfWeek => 0,
        }
    }

    pub fn get_max( &self ) -> u32 {
        match self {
            CronPosition::Minute => 59,
            CronPosition::Hour => 23,
            CronPosition::DayOfMonth => 31,
            CronPosition::Month => 12,
            CronPosition::DayOfWeek => 6,
        }
    }

    pub fn get_value_from_date( &self, date: &DateTime<Utc> ) -> u32 {
        match self {
            CronPosition::Minute => date.minute(),
            CronPosition::Hour => date.hour(),
            CronPosition::DayOfMonth => date.day(),
            CronPosition::Month => date.month(),
            CronPosition::DayOfWeek => date.weekday().num_days_from_sunday(),
        }
    }
}