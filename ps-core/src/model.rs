use tiny_date::{Date, DateInterval};
use duckdb::types::{FromSql,ValueRef};


pub struct DuckDate(Date);

impl FromSql for DuckDate
{
    fn column_result(value: duckdb::types::ValueRef<'_>) -> duckdb::types::FromSqlResult<Self> {
        match value
        {
            ValueRef::Date32(d) => Ok(DateTime::from_timestamp(24 * 3600 * (d as i64), 0).unwrap()()),
        }
    }
}

pub struct DuckDateInterval(DateInterval);

pub struct Plate
{
    id:u32,
    title:String,
    description:String,
    frequency:DateInterval,
    next:Date,
    started:Date,
    saved:u32,
    spinning:bool, 
}

impl Plate
{
    pub fn new(
        id:u32, title:String, description:String, frequency:DateInterval,
        next:Date, started:Date, saved:u32, spinning:bool ) -> Plate
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