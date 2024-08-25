//! ## Authenticator
//!
//! Module for authenticating users who subscribe to notifications. Through authentication, we
//! ensure that the email or Telegram account for which notifications are enabled is authorized by
//! the account owner.

use dotenv::dotenv;
use google_oauth::AsyncClient;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct UserInfo {
	// The email of the user.
	email: String,
}

/// Identifies the user based on the access token.
///
/// When successful returns the user's email.
pub async fn authenticate_google_user(access_token: &str) -> Result<String, &'static str> {
	dotenv().ok();

	let client_id = env::var("CLIENT_ID").unwrap();
	let client = AsyncClient::new(client_id);

	client.validate_access_token(access_token).await.map_err(|_err| "TODO: error")?;
	let user = get_user_info(&access_token).await.unwrap();

	Ok(user.email)
}

pub fn authenticate_telegram_user() -> Result<(), &'static str> {
	Ok(())
}

async fn get_user_info(access_token: &str) -> Result<UserInfo, reqwest::Error> {
	let url = "https://www.googleapis.com/oauth2/v3/userinfo";
	let client = reqwest::Client::new();

	let response = client
		.get(url)
		.header(AUTHORIZATION, format!("Bearer {}", access_token))
		.send()
		.await?;

	let user_info: UserInfo = response.json().await?;
	Ok(user_info)
}
