use tiny_date::{Date, DateInterval, Interval};
use duckdb::types::{FromSql,FromSqlError,ValueRef};


pub struct DuckDate(Date);

impl FromSql for DuckDate
{
    fn column_result(value: duckdb::types::ValueRef<'_>) -> duckdb::types::FromSqlResult<Self> 
    {
        match value
        {
            ValueRef::Date32(d) => 
            {
                if d < 0
                {
                    Err(FromSqlError::OutOfRange(0))
                }
                else
                {
                    match Date::from_timestamp(d as u32)
                    {
                        Ok(da) => Ok(DuckDate(da)),
                        Err(e) => Err(FromSqlError::OutOfRange(0))
                    }
                }
                
            },
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

pub struct DuckDateInterval(DateInterval);

impl FromSql for DuckDateInterval
{
    fn column_result(value: ValueRef<'_>) -> duckdb::types::FromSqlResult<Self> {
        match value
        {
            ValueRef::Interval { months, days, nanos } =>
            {
                if months > 0
                {
                    Ok(DuckDateInterval(DateInterval { amount:months as u32, period:Interval::Month  }))
                }
                else if days > 0
                {
                    Ok(DuckDateInterval(DateInterval { amount:days as u32, period:Interval::Day }))
                }
                else
                {
                    let amount = nanos /  (1_000_000_000 * 60 * 60 * 24);
                    Ok(DuckDateInterval(DateInterval { amount:amount as u32, period:Interval::Day }))
                }
            },
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

pub struct Plate
{
    id:u32,
    title:String,
    description:String,
    frequency:DuckDateInterval,
    next:DuckDate,
    started:DuckDate,
    saved:u32,
    spinning:bool, 
}

impl Plate
{
    pub fn new(
        id:u32, title:String, description:String, frequency:DuckDateInterval,
        next:DuckDate, started:DuckDate, saved:u32, spinning:bool ) -> Plate
    {
        Plate
        {
            id,
            title,
            description,
            frequency,
            next,
            started,
            saved,
            spinning
        }
    }
}