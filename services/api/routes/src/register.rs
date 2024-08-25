use crate::errors::{custom_error, Error};
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
	/// Notifications the user enabled.
	#[serde(rename = "enabledNotifications")]
	pub enabled_notifications: Vec<Notifications>,
	/// Data used to authenticate users.
	#[serde(rename = "authData")]
	pub auth_data: AuthData,
}

/// Contains data to authenticate users when registering.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthData {
	/// Token for authenticating users who wish to receive the notifications via email.
	pub email_access_token: Option<String>,
	/// Token for authenticating users who wish to receive the notifications via telegram.
	pub tg_auth_token: Option<String>,
}

impl RegistrationData {
	fn validate(&self) -> Result<(), Error> {
		// Ensure the configured notifier and auth data is set.
		match &self.notifier {
			Notifier::Email(email) => {
				ensure!(self.auth_data.email_access_token.is_some(), Error::AuthDataEmpty);
			},
			Notifier::Telegram(tg_handle) => {
				ensure!(self.auth_data.tg_auth_token.is_some(), Error::AuthDataEmpty);
			},
			_ => (),
		};

		Ok(())
	}
}

#[post("/register_user", data = "<registration_data>")]
pub async fn register_user(
	conn: &State<DbConn>,
	registration_data: Json<RegistrationData>,
) -> Result<status::Custom<()>, status::Custom<Json<ErrorResponse>>> {
	// Get connection:
	let conn = conn
		.lock()
		.map_err(|_| custom_error(Status::InternalServerError, Error::DbConnectionFailed))?;

	// Validate registration data:
	registration_data
		.validate()
		.map_err(|error| custom_error(Status::BadRequest, error))?;

	ensure_unique_data(&conn, &registration_data)?;

	match registration_data.notifier {
		Notifier::Email(_) => {
			/* TODO:
			let email = authenticator::authenticate_google_user(
				registration_data.auth_data.email_access_token,
			)
			.await?;
			ensure!(
				Some(email) == registration_data.email,
				custom_error(Status::Unauthorized, Error::BadAuthData)
			);
			*/
		},
		Notifier::Telegram(_) => {},
		Notifier::Null => {},
	}

	let user = User { id: registration_data.id, notifier: registration_data.notifier.clone() };
	// Register user
	User::create_user(&conn, &user)
		.map_err(|_| custom_error(Status::InternalServerError, Error::DbError))?;

	Ok(status::Custom(Status::Ok, ()))
}

fn ensure_unique_data(
	conn: &Connection,
	registration_data: &Json<RegistrationData>,
) -> Result<(), status::Custom<Json<ErrorResponse>>> {
	let maybe_user = User::query_by_id(&conn, registration_data.id)
		.map_err(|_| custom_error(Status::InternalServerError, Error::DbError))?;

	// Ensure that the id is unique.
	ensure!(maybe_user.is_none(), custom_error(Status::Conflict, Error::UserExists));

	let error = custom_error(Status::Conflict, Error::NotifierNotUnique);

	if let Notifier::Email(email) = &registration_data.notifier {
		let maybe_user = User::query_by_email(&conn, email.clone())
			.map_err(|_| custom_error(Status::InternalServerError, Error::DbError))?;
		ensure!(maybe_user.is_none(), error);
	}
	if let Notifier::Email(tg_handle) = &registration_data.notifier {
		let maybe_user = User::query_by_tg_handle(&conn, tg_handle.clone())
			.map_err(|_| custom_error(Status::InternalServerError, Error::DbError))?;
		ensure!(maybe_user.is_none(), error);
	}

	Ok(())
}
