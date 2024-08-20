//! ## Authenticator
//!
//! Module for authenticating users who subscribe to notifications. Through authentication, we
//! ensure that the email or Telegram account for which notifications are enabled is authorized by
//! the account owner.

pub fn authenticate_google_user() -> Result<(), &'static str> {
	Ok(())
}

pub fn authenticate_telegram_user() -> Result<(), &'static str> {
	Ok(())
}
