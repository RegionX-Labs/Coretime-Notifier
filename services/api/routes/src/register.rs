use rocket::{http::Status, post, response::status, serde::json::Json, State};
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
	fn validate(&self) -> Result<(), &str> {
		// Ensure the configured notifier is set.
		match self.notifier {
			Notifier::Email if self.email.is_none() => Err("Email must not be empty"),
			Notifier::Telegram if self.tg_handle.is_none() => Err("Telegram handle must exist"),
			_ => Ok(()),
		}
	}
}

#[post("/register_user", data = "<registration_data>")]
pub async fn register_user(
	conn: &State<DbConn>,
	registration_data: Json<RegistrationData>,
) -> Result<status::Custom<()>, status::Custom<Json<ErrorResponse>>> {
	let conn = conn
		.lock()
		.map_err(|_| custom_error(Status::InternalServerError, "DB connection failed"))?;

	registration_data
		.validate()
		.map_err(|error| custom_error(Status::BadRequest, error))?;

	let error = Err(custom_error(
		Status::InternalServerError,
		"User already exists with the same notifier",
	));
	if let Some(email) = registration_data.email.clone() {
		if User::query_by_email(&conn, email).is_ok() {
			return error
		}
	}
	if let Some(tg_handle) = registration_data.tg_handle.clone() {
		if User::query_by_tg_handle(&conn, tg_handle).is_ok() {
			return error
		}
	}

	let user = User {
		id: registration_data.id,
		email: registration_data.email.clone(),
		tg_handle: registration_data.tg_handle.clone(),
		notifier: registration_data.notifier.clone(),
	};
	// Register user
	User::create_user(&conn, &user)
		.map_err(|_| custom_error(Status::InternalServerError, "Failed to register user"))?;

	Ok(status::Custom(Status::Ok, ()))
}

fn custom_error(status: Status, message: &str) -> status::Custom<Json<ErrorResponse>> {
	status::Custom(status, Json(ErrorResponse { message: message.to_string() }))
}
