use crate::{
	errors::Error,
	query::user,
	register::{register_user, AuthData, RegistrationData},
	tests::mock::execute_with,
};
use rocket::{
	http::{ContentType, Status},
	local::blocking::{Client, LocalResponse},
	routes,
};
use serde_json::from_str;
use storage::{init_db, users::User};
use types::{api::ErrorResponse, Notifier};

pub const DB_PATH: &'static str = "query-tests.db";

#[test]
fn register_works() {
	execute_with(DB_PATH, || {
		let conn = init_db(DB_PATH).unwrap();
		let rocket = rocket::build().manage(conn).mount("/", routes![register_user, user]);

		let client = Client::tracked(rocket).expect("failed to create a client");

		// CASE 1: user doesn't exist.
		let response = client.get("/user/0").dispatch();
		assert_eq!(response.status(), Status::BadRequest);
		assert_eq!(parse_err_response(response), Error::UserNotFound);

		// CASE 2: user exists:

		// Register a user:
		let registration_data = RegistrationData {
			id: 0,
			notifier: Notifier::Email("dummy@gmail.com".to_string()),
			enabled_notifications: vec![],
			auth_data: AuthData {
				email_access_token: Some("token".to_string()),
				tg_auth_token: Some("token".to_string()),
			},
		};
		let response = register(&client, &registration_data);
		assert_eq!(response.status(), Status::Ok);

		let response = client.get("/user/0").dispatch();
		// After registering we should be able to get the user:
		assert_eq!(
			parse_ok_response(response),
			User { id: 0, notifier: Notifier::Email("dummy@gmail.com".to_string()) }
		);
	});
}

fn register<'a>(client: &'a Client, data: &'a RegistrationData) -> LocalResponse<'a> {
	client
		.post("/register_user")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&data).unwrap())
		.dispatch()
}

fn parse_ok_response<'a>(response: LocalResponse<'a>) -> User {
	let body = response.into_string().unwrap();
	serde_json::from_str(&body).expect("can't parse value")
}

fn parse_err_response<'a>(response: LocalResponse<'a>) -> Error {
	let body = response.into_string().unwrap();
	let error: ErrorResponse = from_str(&body).unwrap();
	error.message.into()
}
