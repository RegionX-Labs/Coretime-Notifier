use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Eq, PartialEq, Deserialize)]
pub struct ErrorResponse {
	pub message: String,
}

impl From<&str> for ErrorResponse {
	fn from(s: &str) -> ErrorResponse {
		ErrorResponse { message: s.to_string() }
	}
}
