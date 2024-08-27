//! ## Update Route
//!
//! Update route should handle updating the information of the existing user.
//! A user is allowed to update the following information:
//! - User's notifier (set to null to disable notifications)
//! - User's email address
//! - User's telegram handle
//! - User's enabled notifications
//!
//! We ensure the user exists before validating.
//! If the ID exists, then it can be validated.
//!
//! How can we ensure that the owner of the email | tg is making the update?

use crate::{
	errors::{custom_error, Error},
	update, LOG_TARGET,
};

use common_macros::ensure;
use rocket::{http::Status, put, response::status, serde::json::Json, State};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use storage::{users::User, DbConn};
use types::{api::ErrorResponse, Notifier};

// If there is data that should not be updated, then pass current value.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct UpdateData {
	// The ID of the user to update
	pub id: u32,
	// The email address to update to,
	pub email: Option<String>,
	#[serde(rename = "tgHandle")]
	// The telegram handle to update to
	pub tg_handle: Option<String>,
	// The desired notifier to use.
	// If undefined, notifications will be turned off for user
	// Pass current value if not to be updated
	pub notifier: Option<Notifier>,
}

#[put("/update_user", data = "<update_data>")]
pub async fn update_user(
	conn: &State<DbConn>,
	update_data: Json<UpdateData>,
) -> Result<status::Custom<()>, status::Custom<Json<ErrorResponse>>> {
	// validate the data passed
	// Get data to update and serialize
	// Update the data
	log::info!(target: LOG_TARGET, "Update user request {:?}", update_data);

	// Get connection:
	let conn = conn.lock().map_err(|err| {
		log::error!(target: LOG_TARGET, "DB connection failed: {:?}", err);
		custom_error(Status::InternalServerError, Error::DbConnectionFailed)
	})?;

	// Ensure user exists
	let db_user = verify_existing_id(&conn, update_data.id.clone())?;

	let user = User {
		email: update_data.email.clone(),
		tg_handle: update_data.tg_handle.clone(),
		id: update_data.id.clone(),
		notifier: if update_data.notifier.clone().is_some() {
			update_data.notifier.clone().unwrap()
		} else {
			db_user.notifier
		},
	};	
	let result = User::update(&conn, &user); 

	match result {
		Ok(_) => Ok(status::Custom(Status::Ok, ())),
		Err(_) => Err(custom_error(Status::InternalServerError, Error::DbError)),
	}
}

fn verify_existing_id(
	conn: &Connection,
	user_id: u32,
) -> Result<User, status::Custom<Json<ErrorResponse>>> {
	let maybe_user = User::query_by_id(&conn, user_id).map_err(|err| {
		log::error!(target: LOG_TARGET, "Failed to search user by id: {:?}", err);
		custom_error(Status::InternalServerError, Error::DbError)
	})?;

	match maybe_user {
		Some(user) => Ok(user),
		None => Err(custom_error(Status::NotFound, Error::UserNotFound)),
	}
}
