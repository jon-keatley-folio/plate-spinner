pub mod model;
pub mod plate_data;
mod schema;

//use duckdb::Connection
//use duck::{DBError,Action,List,get_connection};

/* TODO
- [x] Test lists
- [x] Create lib interface
*/

use duckdb::Connection;

use crate::model::Plate;

use crate::plate_data::DBError;
use crate::plate_data::connect;
use crate::plate_data::{Action, List};

pub struct PlateSpinnerCore {
    conn: Connection,
}

impl PlateSpinnerCore {
    pub fn new(conn_uri: &str) -> Result<PlateSpinnerCore, DBError> {
        let c = connect(conn_uri)?;

        Ok(PlateSpinnerCore { conn: c })
    }

    pub fn execute_list(&self, list: List) -> Result<Vec<Plate>, DBError> {
        list.execute(&self.conn)
    }

    pub fn execute_action(&self, action: Action) -> Result<bool, DBError> {
        action.execute(&self.conn)
    }
}
