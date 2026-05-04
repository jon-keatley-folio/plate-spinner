use duckdb::{ffi::duckdb_result, params, Connection, Error, Result};

use tiny_date::{Date,DateInterval};

use crate::schema::{PS_V1_SCHEMA, VALIDATE_SCHEMA_PSV1};


#[derive(PartialEq, Debug, Clone)]
pub(crate) enum DBError
{
    FailedToConnect,
    FailedToCompileQuery,
    UnexpectedResults,
    UnableToCreateSchema,
    UnableToCheckSchema
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum Action
{
    AddPlate(String,String,DateInterval, Date),
    UpdatePlate(String,String,DateInterval,u32),
    PausePlate(u32),
    StartPlateSpinning(u32),
    SpinPlate(u32), 
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum List
{
    TopTopples(u32),
    PausedPlates(u32,u32),
    All(u32,u32)
}



pub(crate) fn get_connection(con_uri:&str) -> Result<Connection, DBError>
{
    let conn = if con_uri == "memory"
    {
         Connection::open_in_memory()
    }
    else
    {
        Connection::open(con_uri)
    };
    
    match conn
    {
        Ok(c) => return Ok(c),
        Err(_) => return Err(DBError::FailedToConnect)
    }
}

fn setup_schema(conn:&Connection) -> bool
{
    let result = conn.execute_batch(PS_V1_SCHEMA);
    result.is_ok()
}

fn check_latest_schema(conn:&Connection) -> Result<bool, DBError>
{
    let stmt_result = conn.prepare(VALIDATE_SCHEMA_PSV1);
    
    if let Ok(mut stmt) = stmt_result
    {
        let duckdb_result:Result<String,Error> = stmt.query_one([], |row| {
            row.get(0)
        });
        
        match duckdb_result
        {
            Ok(schema) => return Ok(schema == "psv1"),
            Err(_) => return Ok(false)
        }
    }
    
    Err(DBError::UnableToCheckSchema)
}

pub(crate) fn check_or_create_schema(conn:&Connection) -> Result<bool, DBError>
{
    match check_latest_schema(conn)
    {
        Ok(true) => return Ok(true),
        Ok(false) =>
        {
            if setup_schema(conn)
            {
                return Ok(true)
            }
            else
            {
                return Err(DBError::UnableToCreateSchema)
            }
        },
        Err(e) => return Err(e)
    }
}

#[cfg(test)]
mod tiny_the_ducks {
    use super::*;

    #[test]
    fn test_create_connection()
    {
        let result = get_connection("memory");
        
        assert!(result.is_ok())
    }
    
    #[test]
    fn test_setup_schema_and_schema_check()
    {
        let conn = get_connection("memory").unwrap();
        
        let schema_check_one = check_latest_schema(&conn);
        
        assert!(schema_check_one.is_ok());
        assert!(!schema_check_one.unwrap());
        
        let results = setup_schema(&conn);
        
        assert!(results);
        
        let schema_check_two = check_latest_schema(&conn);
        
        assert!(schema_check_two.is_ok());
        assert!(schema_check_two.unwrap());
    }
    
}

