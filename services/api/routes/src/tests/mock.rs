use rocket::local::blocking::LocalResponse;
use storage::users::User;
use types::api::ErrorResponse;

use crate::errors::Error;

pub fn execute_with<R>(db_path: &str, f: impl Fn() -> R) -> R {
	// Don't check the result since it will error if the db already doesn't exist which isn't an
	// issue.
	let _ = std::fs::remove_file(db_path);

	f()
}

pub fn parse_ok_response<'a>(response: LocalResponse<'a>) -> User {
	let body = response.into_string().unwrap();
	serde_json::from_str(&body).expect("can't parse value")
}

pub fn parse_err_response<'a>(response: LocalResponse<'a>) -> Error {
	let body = response.into_string().unwrap();
	let error: ErrorResponse = serde_json::from_str(&body).unwrap();
	error.message.into()
}
