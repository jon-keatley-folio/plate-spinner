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

#[derive(PartialEq, Debug)]
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

pub enum Interval
{
    Day,
    Month,
    Year,
}

pub struct DateInterval
{
    pub amount:u32,
    pub period:Interval
}


#[derive(Clone)]
pub struct Date
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
            if year % 400 == 0
            {
                return true;
            }
        }
        else
        {
            return true;
        }
    }
    
    false
}

fn get_max_days_for_month(year:u32, month:u32) -> u32
{
    if is_leap_year(year) && month == 2
    {
        NUMBER_OF_DAYS[(month -1) as usize] + 1u32
    }
    else
    {
        NUMBER_OF_DAYS[(month -1) as usize]
    }
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
        
        let max_days_for_month = get_max_days_for_month(year, month);
        
        if day < 1 || day > max_days_for_month
        {
            return Err(DateError::InvalidDate);
        }
        
        Ok(Date{day, month, year})

    }
    
    pub fn day(&self) -> u32
    {
        self.day
    }
    
    pub fn month(&self) -> u32
    {
        self.month
    }
    
    pub fn year(&self) -> u32
    {
        self.year
    }
    
    pub fn apply_interval(&self, interval:DateInterval) -> Result<Date, DateError>
    {
        match interval.period
        {
            Interval::Year => {
                Date::new(self.day, self.month, self.year + interval.amount)
            },
            Interval::Month => {
                let mut new_month = self.month + interval.amount;
                let new_year = if new_month > 12
                {
                    new_month -= 12;
                    self.year + 1
                }
                else
                {
                    self.year
                };
                
                let max_days_this_month = get_max_days_for_month(new_year, new_month);
                let new_day = if max_days_this_month >= self.day
                {
                    self.day
                }
                else
                {
                    new_month += 1;
                    self.day - max_days_this_month
                };
                
                Date::new(new_day, new_month , new_year)
            },
            Interval::Day => {
                let mut new_day = self.day + interval.amount;
                let mut new_month = self.month;
                let mut new_year = self.year;
                
                let max_days_for_month = get_max_days_for_month(new_year, new_month);
                
                if new_day > max_days_for_month
                {
                    new_day = new_day - max_days_for_month;
                    new_month += 1;
                    if new_month > 12
                    {
                        new_month = 1;
                        new_year += 1;
                    }
                }
                
                Date::new(new_day, new_month, new_year)
            }
        }
    }
}

impl fmt::Display for Date
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}-{}-{}",  self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod tiny_dates_tests {
    use super::*;

    #[test]
    fn create_date_happy() {
        let result = Date::new(7,5,2019);
        
        assert!(result.is_ok());
        
        if let Ok(d) = result
        {
            assert_eq!(d.day(), 7);
            assert_eq!(d.month(), 5);
            assert_eq!(d.year(), 2019);
        }
    }
    
    #[test]
    fn create_date_sad() {
        let tests = [
            (0,1,2020,DateError::InvalidDate),
            (1,0,2020,DateError::InvalidMonth),
            (1,1,0,DateError::InvalidYear),
            (1,1,1751,DateError::InvalidYear),
        ];
        
        for t in tests
        {
            let tr = Date::new(t.0,t.1,t.2);
            
            assert!(tr.is_err());
            if let Err(err) = tr
            {
                assert_eq!(err, t.3);
            }
        }
    }
    
    #[test]
    fn create_date_on_leap_year() {
        let tests = [
            2000, 2004, 2008, 2012, 2016, 2020, 2024, 2028, 2032, 2036, 2040, 2044,
            2048, 2052, 2056, 2060, 2064, 2068, 2072, 2076, 2080, 2084, 2088, 2092, 
            2096
        ];
        
        for year in tests
        {
            let tr = Date::new(29,2,year);
            assert!(tr.is_ok());
        }
    }
    
    #[test]
    fn create_date_when_not_leap_year() {
        let tests = [
            2001, 2005, 2009, 2013, 2017,
        ];
        
        for year in tests
        {
            let tr = Date::new(29,2,year);
            assert!(tr.is_err());
        }
    }
    
    #[test]
    fn test_day_interval()
    {
        let start_day = Date::new(10,1,2004).unwrap();
        let leap_day = Date::new(28,2,2004).unwrap();
        
        let test_day = start_day.apply_interval(
            DateInterval{
                amount:10,
                period:Interval::Day
            }
        );
        
        assert!(test_day.is_ok());
        
        if let Ok(result) = test_day
        {
            assert_eq!(result.day(), 20);
            assert_eq!(result.month(), 1);
            assert_eq!(result.year(), 2004);
        }
        
        let test_roll_over = start_day.apply_interval(
            DateInterval { amount: 22, period: Interval::Day }
        );
        
        assert!(test_roll_over.is_ok());
        
        if let Ok(result) = test_roll_over
        {
            assert_eq!(result.day(), 1);
            assert_eq!(result.month(), 2);
            assert_eq!(result.year(), 2004);
        }
        
        let test_leap = leap_day.apply_interval(
            DateInterval { amount: 1, period: Interval::Day }
            );
        
        assert!(test_leap.is_ok());
        if let Ok(result) = test_leap
        {
            assert_eq!(result.day(), 29);
        }
        
    }
    
    #[test]
    fn test_month_interval()
    {
        let test_month = Date::new(31,1,2025).unwrap();
        
        let test_roll_over = test_month.apply_interval(
            DateInterval { amount: 1, period: Interval::Month }
        );
        
        assert!(test_roll_over.is_ok());
        
        if let Ok(result) = test_roll_over
        {
            assert_eq!(result.day(), 3);
            assert_eq!(result.month(), 3);
            assert_eq!(result.year(), 2025);
        }
    }
    
    #[test]
    fn test_year_interval()
    {
        let test_year = Date::new(31,1,2025).unwrap();
        
        let test = test_year.apply_interval(
            DateInterval { amount: 10, period: Interval::Year }
        );
        
        assert!(test.is_ok());
        
        if let Ok(result) = test
        {
            assert_eq!(result.day(), 31);
            assert_eq!(result.month(), 1);
            assert_eq!(result.year(), 2035);
        }
    }
}