use crate::register::{register_user, RegistrationData};
use rocket::{http::ContentType, local::blocking::Client, routes};
use types::Notifier;

#[test]
fn register_works() {
	let rocket = rocket::build().mount("/", routes![register_user]);
	let client = Client::tracked(rocket).expect("failed to create a client");

	let registration_data = RegistrationData {
		id: 0,
		notifier: Notifier::Email,
		email: Some("dummy@gmail.com".to_string()),
		tg_handle: None,
		enabled_notifications: vec![],
	};

	let response = client
		.post("/register_user")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&registration_data).unwrap())
		.dispatch();

	assert_eq!(response.status(), Status::Ok);
}
