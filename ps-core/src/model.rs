use tiny_date::{Date, DateInterval};
use duckdb::types::{FromSql,FromSqlError,ValueRef};


pub struct DuckDate(Date);

impl FromSql for DuckDate
{
    fn column_result(value: duckdb::types::ValueRef<'_>) -> duckdb::types::FromSqlResult<Self> {
        
        
        match value
        {
            ValueRef::Date32(d) => 
            
                match DuckDate::from_timestamp(d)
                {
                    Ok(d) => Ok(d),
                    Err(e) => Err(FromSqlError::OutOfRange(0))
                }
            },
            _ => Err(FromSqlError::InvalidType)
        }
    }
}

pub struct DuckDateInterval(DateInterval);

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