use crate::errors::{custom_error, Error};
use rocket::{get, http::Status, response::status, serde::json::Json, State};
use storage::{users::User, DbConn};
use types::api::ErrorResponse;

#[get("/user/<user_id>")]
pub async fn user(
	conn: &State<DbConn>,
	user_id: u32,
) -> Result<status::Custom<String>, status::Custom<Json<ErrorResponse>>> {
	let conn = conn
		.lock()
		.map_err(|_| custom_error(Status::InternalServerError, Error::DbConnectionFailed))?;

	let maybe_user = User::query_by_id(&conn, user_id)
		.map_err(|_| custom_error(Status::InternalServerError, Error::DbError))?;

	let Some(user) = maybe_user else {
		return Err(custom_error(Status::BadRequest, Error::UserNotFound));
	};

	// TODO: log errors
	let serialized = serde_json::to_string(&user)
		.map_err(|_| custom_error(Status::InternalServerError, Error::FailedToSerialize))?;

	Ok(status::Custom(Status::Ok, serialized))
}
