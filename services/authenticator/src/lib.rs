//! ## Authenticator
//!
//! Module for authenticating users who subscribe to notifications. Through authentication, we
//! ensure that the email or Telegram account for which notifications are enabled is authorized by
//! the account owner.

use google_oauth::AsyncClient;
use serde::Deserialize;
use reqwest::header::AUTHORIZATION;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize)]
struct UserInfo {
    email: String,
}

pub async fn authenticate_google_user(access_token: &str) -> Result<(), &'static str> {
    dotenv().ok();

	let client_id = env::var("CLIENT_ID").unwrap();
    println!("{}", client_id);

	let client = AsyncClient::new(client_id);

    let payload = client.validate_access_token(access_token).await.unwrap(); // In production, remember to handle this error.

    println!("Hello {}", &payload.sub);
	let user = get_user_info(&access_token).await.unwrap();
	println!("email: {}", user.email);

	Ok(())
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
