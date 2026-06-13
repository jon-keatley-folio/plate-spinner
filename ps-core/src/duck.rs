use std::ops::Deref;

use duckdb::{params, Connection, Error, Result, ToSql};
use tiny_date::{Date,DateInterval, Interval};

use crate::schema::{PS_V1_SCHEMA, VALIDATE_SCHEMA_PSV1, LATEST_VERSION};


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

impl Action
{
    fn get_prepared_statement(&self) -> String
    {
        let sql="EXECUTE";
        match self
        {
            Self::AddPlate(_,_,_,_ ) => format!("{} {}", sql,"add_plate"),
            Self::UpdatePlate(_,_,_,_) => format!("{} {}",sql,"update_plate"),
            Self::PausePlate(_) => format!("{} {}",sql,"pause_plate"),
            Self::StartPlateSpinning(_) => format!("{} {}",sql,"start_spinning_plate"),
            Self::SpinPlate(_) => format!("{} {}",sql,"spin_plate")
        }
    }
    
    fn execute(&self, conn:&Connection) -> Result<bool,DBError>
    {
        let params = match self
        {
            Self::AddPlate(t,d,f,n ) => 
            {
                format!("('{}','{}','{}','{}')",t, d, f.to_string(), n.to_string())
            },
            Self::UpdatePlate(t,d,f,id) => 
            {
                let interval = f.to_string();
                format!("('{}','{}','{}',{})",t.clone(),d.clone(),interval.clone(),id.clone())
            },
            Self::PausePlate(id) => format!("({})",id.clone()),
            Self::StartPlateSpinning(id) => format!("({})",id.clone()),
            Self::SpinPlate(id) => format!("({})",id.clone())
        };
        
        let query = format!(
            "{}{}",
            self.get_prepared_statement(),
            params
        );
        let prep = conn.prepare(&query);
        match prep
        {
            Ok(mut p) =>
            {
                match p.execute([])
                {
                    Ok(change) => return if change > 0 
                    {
                        Ok(true)
                    }
                    else
                    {
                        Err(DBError::UnexpectedResults)
                    },
                    Err(e) =>
                    {
                        println!("ERROR: {}, {}",e, self.get_prepared_statement());
                        return Err(DBError::UnexpectedResults)
                    }
                }
            },
            Err(e) => 
            {
                println!("Failed to prepare Action. {}",e);
                return Err(DBError::FailedToCompileQuery)
            }
        }  
    }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum List
{
    TopTopples(u32),
    PausedPlates(u32),
    All(u32,u32)
}

impl List
{
    fn get_prepared_statement(&self) -> String
    {
        let sql="EXECUTE";
        match self
        {
            Self::TopTopples(_) => format!("{} top_topples(?)", sql),
            Self::PausedPlates(_) => format!("{} paused_plates(?)", sql),
            Self::All(_,_) => format!("{} list_places(?,?)", sql)
        }
    }
    
    pub fn execute(&self, conn: &Connection) -> Result<bool, DBError>
    {
        let params = match self
        {
            Self::TopTopples(limit) => params![limit.clone()],
            Self::PausedPlates(limit) => params![limit.clone()],
            Self::All(limit,offset ) => params![limit.clone(), offset.clone()]
        };
        
        let result = conn.execute(
            &self.get_prepared_statement(),
            params);
        
       // match result
        //{
       //     Ok(rows)
       // }
        if result.is_ok()
        {
            Ok(true)
        }
        else
        {
            Err(DBError::UnexpectedResults)
        }
    }
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

pub(crate) fn select_version(conn:&Connection) -> Result<(),DBError>
{
    match conn.execute(&format!("use {}",LATEST_VERSION), [])
    {
        Ok(_) => Ok(()),
        Err(_) => Err(DBError::FailedToConnect)
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
    
    #[test]
    fn test_actions()
    {
        let conn = get_connection("memory").unwrap();
        let results = setup_schema(&conn);
        assert!(results);
        assert!(select_version(&conn).is_ok());
        
        //add
        let date = Date::new(10,1,2025).unwrap();
        let add = Action::AddPlate(
            "test title".to_string(),
            "this is a test".to_string(),
            DateInterval{
                amount:2,
                period:Interval::Day
            },
            date
        );
        
        let add_result = add.execute(&conn);
        assert!(add_result.is_ok());
        
        //update
        let edit = Action::UpdatePlate(
            "title update".to_string(),
            "this is a test p2".to_string(),
            DateInterval { amount: 3, period: Interval::Day },
            1u32
        );
        
        let edit_result = edit.execute(&conn);
        assert!(edit_result.is_ok());
        
        let pause = Action::PausePlate(1u32);
        let pause_result = pause.execute(&conn);
        assert!(pause_result.is_ok());
        
        let unpause = Action::StartPlateSpinning(1u32);
        let unpause_result = unpause.execute(&conn);
        assert!(unpause_result.is_ok());
        
        let spin = Action::SpinPlate(1u32);
        let spin_result = spin.execute(&conn);
        assert!(spin_result.is_ok());
        
    }
    
    fn add_test_plate(step:u32, conn:&Connection)
    {
        
        let mut date = Date::new(10,1,2025).unwrap();
        let interval = DateInterval{
                amount:step,
                period:Interval::Day
        };
        date = date.apply_interval(interval.clone()).unwrap();
            
        let add = Action::AddPlate(
            format!("test plate {}", step),
            "this is a test".to_string(),
            interval,
            date
        );
        
        let result = add.execute(conn);
        assert!(result.is_ok());
        
    }
    
    #[test]
    fn test_listing()
    {
        let conn = get_connection("memory").unwrap();
        let results = setup_schema(&conn);
        assert!(results);
        assert!(select_version(&conn).is_ok());
        
        for i in 1..10
        {
            add_test_plate(i, &conn);
        }
        
        let paused_plates = [3u32,6u32,7u32];
        for i in paused_plates
        {
            let p = Action::PausePlate(i);
            let result = p.execute(&conn);
            assert!(result.is_ok());
        }
        
        
        //TopTopples(u32),
        let top = List::TopTopples(3u32);
        let top_results = top.execute(&conn);
        assert!(top_results.is_ok());
        //PausedPlates(u32),
        //All(u32,u32)
        
        //add 
    }
    
}

