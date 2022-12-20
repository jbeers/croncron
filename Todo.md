# Todo



add W support for day of month
add L support for day of month and day of week
Double check next occurrence logic
Add function to generate next x occurrences
Add a macro to allow parsing cron things like cron!( * * * * * )

# Completed
add function to test if date matches cron schedule
Add number support to each position
Add number validation for each position
Add - support with validation
Changes functionality to be focused on checking if date is occurrence of schedule (add next_occurrence logic later)
Add text support to Monday and day of week
add / interval support

# Notes

Should CronArg be a Trait?
    impl CronArg for 

Should the date reset logic be implemented
    as a macro
    as a new trait ResettableDate implemented for DateTime
    as a function in CronPosition `setDate( date:DateTime, value )` 
        I like this I went on to add next_value to position
        by taking this approach I may be able to drop the ever growing match statement in CronArg.update_date