use rocket::serde::{Deserialize, Serialize};
use std::fmt;

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
			_ => panic!("UnknownError"),
		}
	}
}
