use std::sync::{Arc, Mutex, MutexGuard};

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, Outcome};

use mytable::Table;

use crate::person::Person;


static mut TABLE_CONN: Option<MytableConnection> = None;


#[derive(Debug)]
pub struct MytableConnection {
    pub table_arc: Arc<Mutex<Table>>
}


impl MytableConnection {
    pub fn new() -> Self {
        Self {
            table_arc: Arc::new(Mutex::new(
                Table::new::<Person>("person.tbl")
            ))
        }
    }

    pub fn lock_table(&self) -> MutexGuard<Table> {
        self.table_arc.lock().unwrap()
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for &'a MytableConnection {
    type Error = ();

    fn from_request(
                _request: &'a Request<'r>
            ) -> request::Outcome<&'a MytableConnection, ()> {
        unsafe {
            match TABLE_CONN.as_mut() {
                Some(conn) => Outcome::Success(conn),
                None => Outcome::Failure((Status::ServiceUnavailable, ()))
            }
        }
    }
}


pub fn connect() {
    unsafe {
        TABLE_CONN = Some(MytableConnection::new());
    }
}
