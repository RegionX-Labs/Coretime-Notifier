use rocket::{
	http::Status,
	response::status,
	serde::{json::Json, Deserialize, Serialize},
};
use std::fmt;
use types::api::ErrorResponse;

/// Standardized errors that the API might return.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Error {
	/// Failed to get the db connection.
	DbConnectionFailed,
	/// The configured notifier cannot be empty.
	NotifierEmpty,
	/// Attempted accessing the db but failed.
	DbError,
	/// User with the same id exists.
	UserExists,
	/// The notifier is already in use by some other user.
	NotifierNotUnique,
	/// The user doesn't exist.
	UserNotFound,
	/// Failed to serialize some data,
	FailedToSerialize,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl From<String> for Error {
	fn from(s: String) -> Self {
		match s.as_str() {
			"DbConnectionFailed" => Error::DbConnectionFailed,
			"NotifierEmpty" => Error::NotifierEmpty,
			"DbError" => Error::DbError,
			"UserExists" => Error::UserExists,
			"NotifierNotUnique" => Error::NotifierNotUnique,
			"UserNotFound" => Error::UserNotFound,
			"FailedToSerialize" => Error::FailedToSerialize,
			_ => panic!("UnknownError"),
		}
	}
}

pub fn custom_error(status: Status, error: Error) -> status::Custom<Json<ErrorResponse>> {
	status::Custom(status, Json(ErrorResponse { message: error.to_string() }))
}
