use crate::{
	errors::{custom_error, Error},
	LOG_TARGET,
};
use rocket::{get, http::Status, response::status, serde::json::Json, State};
use storage::{users::User, DbConn};
use types::api::ErrorResponse;

#[get("/user/<user_id>")]
pub async fn user(
	conn: &State<DbConn>,
	user_id: u32,
) -> Result<status::Custom<String>, status::Custom<Json<ErrorResponse>>> {
	log::info!(target: LOG_TARGET, "Querying user: {}", user_id);

	let conn = conn.lock().map_err(|err| {
		log::error!(target: LOG_TARGET, "DB connection failed: {:?}", err);
		custom_error(Status::InternalServerError, Error::DbConnectionFailed)
	})?;

	let maybe_user = User::query_by_id(&conn, user_id).map_err(|err| {
		log::error!(target: LOG_TARGET, "Failed to search user by id: {:?}", err);
		custom_error(Status::InternalServerError, Error::DbError)
	})?;

	let Some(user) = maybe_user else {
		return Err(custom_error(Status::BadRequest, Error::UserNotFound));
	};

	let serialized = serde_json::to_string(&user).map_err(|err| {
		log::error!(target: LOG_TARGET, "Failed to serialize: {:?}", err);
		custom_error(Status::InternalServerError, Error::FailedToSerialize)
	})?;

	log::info!(target: LOG_TARGET, "Queried User: {:?}", serialized);
	Ok(status::Custom(Status::Ok, serialized))
}
