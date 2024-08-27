use crate::{
	errors::{custom_error, Error},
	LOG_TARGET,
};
use common_macros::ensure;
use rocket::{http::Status, post, response::status, serde::json::Json, State};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use storage::DbConn;
use types::{api::ErrorResponse, Notifications, Notifier};

use storage::users::User;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationData {
	// TODO, for now we are using a u32 for identification, however, this will likely change once
	// we do some form of user authentication.
	pub id: u32,
	/// Defines how the user wants to receive their notifications.
	pub notifier: Notifier,
	// The user's email, will be used if notifier == `Notifier::Telegram`
	pub email: Option<String>,
	// The user's telegram handle, used if tg_handle == `Notifier::Email`
	#[serde(rename = "tgHandle")]
	pub tg_handle: Option<String>,
	// Notifications the user enabled.
	#[serde(rename = "enabledNotifications")]
	pub enabled_notifications: Vec<Notifications>,
}

impl RegistrationData {
	fn validate(&self) -> Result<(), Error> {
		// Ensure the configured notifier is set.
		match self.notifier {
			Notifier::Email if self.email.is_none() => Err(Error::NotifierEmpty),
			Notifier::Telegram if self.tg_handle.is_none() => Err(Error::NotifierEmpty),
			_ => Ok(()),
		}
	}
}

#[post("/register_user", data = "<registration_data>")]
pub async fn register_user(
	conn: &State<DbConn>,
	registration_data: Json<RegistrationData>,
) -> Result<status::Custom<()>, status::Custom<Json<ErrorResponse>>> {
	log::info!(target: LOG_TARGET, "Registration request: {:?}", registration_data);

	// Get connection:
	let conn = conn.lock().map_err(|err| {
		log::error!(target: LOG_TARGET, "DB connection failed: {:?}", err);
		custom_error(Status::InternalServerError, Error::DbConnectionFailed)
	})?;

	// Validate registration data:
	registration_data
		.validate()
		.map_err(|error| custom_error(Status::BadRequest, error))?;

	ensure_unique_data(&conn, &registration_data)?;

	let user = User {
		id: registration_data.id,
		email: registration_data.email.clone(),
		tg_handle: registration_data.tg_handle.clone(),
		notifier: registration_data.notifier.clone(),
	};
	// Register user
	User::create_user(&conn, &user).map_err(|err| {
		log::error!(target: LOG_TARGET, "Failed to create user: {:?}", err);
		custom_error(Status::InternalServerError, Error::DbError)
	})?;

	Ok(status::Custom(Status::Ok, ()))
}

fn ensure_unique_data(
	conn: &Connection,
	registration_data: &Json<RegistrationData>,
) -> Result<(), status::Custom<Json<ErrorResponse>>> {
	let error = custom_error(Status::Conflict, Error::NotifierNotUnique);

	if let Some(email) = registration_data.email.clone() {
		let maybe_user = User::query_by_email(&conn, email).map_err(|err| {
			log::error!(target: LOG_TARGET, "Failed to search user by email: {:?}", err);
			custom_error(Status::InternalServerError, Error::DbError)
		})?;
		ensure!(maybe_user.is_none(), error);
	}

	if let Some(tg_handle) = registration_data.tg_handle.clone() {
		let maybe_user = User::query_by_tg_handle(&conn, tg_handle).map_err(|err| {
			log::error!(target: LOG_TARGET, "Failed to search user by telegram: {:?}", err);
			custom_error(Status::InternalServerError, Error::DbError)
		})?;
		ensure!(maybe_user.is_none(), error);
	}

	Ok(())
}
