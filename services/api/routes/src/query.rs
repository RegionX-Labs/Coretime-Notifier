use rocket::{get, http::Status, response::status, serde::json::Json, State};
use storage::{users::User, DbConn};
use types::api::ErrorResponse;

#[get("/user/<user_id>")]
pub async fn user(
	conn: &State<DbConn>,
	user_id: u32,
) -> Result<status::Custom<String>, status::Custom<Json<ErrorResponse>>> {
	let conn = conn.lock().unwrap(); // TODO: don't unwrap

	let user = User::query_by_id(&conn, user_id).map_err(|_| {
		status::Custom(
			Status::InternalServerError,
			Json(ErrorResponse { message: "Failed to find user".to_string() }),
		)
	})?;

	// TODO: log errors
	let serialized = serde_json::to_string(&user).map_err(|_| {
		status::Custom(
			Status::InternalServerError,
			Json(ErrorResponse { message: "Failed to serialize user".to_string() }),
		)
	})?;

	Ok(status::Custom(Status::Ok, serialized))
}
