//! ## Storage Service
//!
//! Responsible for storing the notification configurations of users.
/*
The storage structure should be as follows:

Each user can have multiple notifications enabled. These notifications must be picked
from the `Notifications` enum. (There cannot be duplicates)

*/
use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub mod users;

pub type DbConn = Mutex<Connection>;

pub fn init_db(db_path: &'static str) -> Result<DbConn> {
	// Create the db if it does not exist.
	let conn = Connection::open(db_path)?;
	conn.execute(
		"CREATE TABLE IF NOT EXISTS users (
               id INTEGER PRIMARY KEY NOT NULL,
               tg_handle TEXT UNIQUE,
               email TEXT UNIQUE,
               notifier TEXT CHECK (
                   notifier IN ('email', 'telegram') 
                   OR notifier IS NULL
               )
           )",
		(),
	)?;

	Ok(Mutex::new(conn))
}
