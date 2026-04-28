use std::fmt;
use std::fmt::Formatter;

pub enum Months
{
    January=1,
    February=2,
    March=3,
    April=4,
    May=5,
    June=6,
    July=7,
    August=8,
    September=9,
    October=10,
    November=11,
    December=12
}

pub enum DateError
{
    InvalidDate,
    InvalidMonth,
    InvalidYear
}

const NUMBER_OF_DAYS:[u32; 12]= [
    31u32,28u32,31u32,30u32,31u32,30u32,
    31u32,31u32,30u32,31u32,30u32,31u32
];

struct Date
{
    day:u32,
    month:u32,
    year:u32
}

fn is_leap_year(year:u32) -> bool
{
    if year % 4 == 0
    {
        if year % 100 == 0
        {
            return true;
        }
        else if year % 400 == 0
        {
            return true;
        }
    }
    
    false
}

impl Date
{
    pub fn new(day:u32,month:u32,year:u32) -> Result<Date,DateError>
    {
        if year < 1752
        {
            return Err(DateError::InvalidYear);
        }
        
        if month < 1 || month > 12
        {
            return Err(DateError::InvalidMonth);
        }
        
        let max_days_for_month = if is_leap_year(year) && month == 2
        {
            NUMBER_OF_DAYS[(month -1) as usize] + 1u32
        }
        else
        {
            NUMBER_OF_DAYS[(month -1) as usize]
        };
        
        if day < 1 || day > max_days_for_month
        {
            return Err(DateError::InvalidDate);
        }
        
        Ok(Date{day, month, year})

    }
    
}

impl fmt::Display for Date
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}-{}-{}",  self.year, self.month, self.day)
    }
}
