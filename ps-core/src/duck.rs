use duckdb::{params, Connection, Result, Error};

use crate::schema::PS_V1_SCHEMA;
/*
get connection


*/

pub(crate) fn get_connection(con_uri:String) -> Result<Connection, Error>
{
    if con_uri == "memory"
    {
        Connection::open_in_memory()
    }
    else
    {
        Connection::open(con_uri)
    }
}

pub(crate) fn setup_schema(conn:Connection) -> bool
{
    let result = conn.execute(PS_V1_SCHEMA, params![]);
    result.is_ok()
}

// execute schema

