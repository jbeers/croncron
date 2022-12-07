use std::fmt::Display;


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