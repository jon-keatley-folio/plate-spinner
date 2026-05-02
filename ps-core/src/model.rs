use tiny_date::{Date, DateInterval};

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