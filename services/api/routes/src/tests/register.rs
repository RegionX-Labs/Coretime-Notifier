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

pub const DB_PATH: &'static str = "registration-tests.db";

#[test]
#[should_panic]
fn no_db_connection_errors() {
	// Should panic because no db connection provided.
	let rocket = rocket::build().mount("/", routes![register_user, user]);
	let _client = Client::tracked(rocket).expect("failed to create a client");
}

#[test]
fn register_works() {
	execute_with(DB_PATH, || {
		let conn = init_db(DB_PATH).unwrap();
		let rocket = rocket::build().manage(conn).mount("/", routes![register_user, user]);

		let client = Client::tracked(rocket).expect("failed to create a client");

		let dummy_auth_data = AuthData {
			email_access_token: Some("token".to_string()),
			tg_auth_token: Some("token".to_string()),
		};

		let mut registration_data = RegistrationData {
			id: 0,
			notifier: Notifier::Null,
			enabled_notifications: vec![],
			auth_data: dummy_auth_data.clone(),
		};
		// CASE 1: the user did not set the notifier.
		let response = register(&client, &registration_data);

		assert_eq!(response.status(), Status::BadRequest);
		assert_eq!(parse_err_response(response), Error::NotifierEmpty);

		// CASE 2: correct data, should work.
		registration_data.notifier = Notifier::Email("dummy@gmail.com".to_string());

		let response = register(&client, &registration_data);
		assert_eq!(response.status(), Status::Ok);

		let response = client.get("/user/0").dispatch();
		// After registering we should be able to get the user:
		assert_eq!(
			parse_ok_response(response),
			User { id: 0, notifier: Notifier::Email("dummy@gmail.com".to_string()) }
		);

		// CASE 3: user with the same id exists
		let response = register(&client, &registration_data);
		assert_eq!(response.status(), Status::Conflict);
		assert_eq!(parse_err_response(response), Error::UserExists);

		// CASE 4: user with the same email exists:
		let registration_data = RegistrationData {
			id: 1,
			notifier: Notifier::Email("dummy@gmail.com".to_string()),
			enabled_notifications: vec![],
			auth_data: dummy_auth_data.clone(),
		};
		let response = register(&client, &registration_data);

		assert_eq!(response.status(), Status::Conflict);
		assert_eq!(parse_err_response(response), Error::NotifierNotUnique);

		// CASE 5: user with the same telegram exists:

		// First time works because there is no user with the same tg:
		let tg_user_1 = RegistrationData {
			id: 2,
			notifier: Notifier::Telegram("@dummy".to_string()),
			enabled_notifications: vec![],
			auth_data: dummy_auth_data.clone(),
		};
		let response = register(&client, &tg_user_1);
		assert_eq!(response.status(), Status::Ok);

		// Second time fails:
		let tg_user_2 = RegistrationData {
			id: 3,
			notifier: Notifier::Telegram("@dummy".to_string()),
			enabled_notifications: vec![],
			auth_data: dummy_auth_data.clone(),
		};
		let response = register(&client, &tg_user_2);
		// assert_eq!(response.status(), Status::Conflict);
		assert_eq!(parse_err_response(response), Error::NotifierNotUnique);
	});
}

#[test]
fn register_fails_without_auth_data() {
	execute_with(DB_PATH, || {
		let conn = init_db(DB_PATH).unwrap();
		let rocket = rocket::build().manage(conn).mount("/", routes![register_user, user]);

		let client = Client::tracked(rocket).expect("failed to create a client");

		let mut registration_data = RegistrationData {
			id: 0,
			notifier: Notifier::Email("dummy@gmail.com".to_string()),
			enabled_notifications: vec![],
			auth_data: AuthData {
				email_access_token: Some("token".to_string()),
				tg_auth_token: Some("token".to_string()),
			},
		};

		// CASE 1: email auth not set
		registration_data.auth_data.email_access_token = None;
		let response = register(&client, &registration_data);

		assert_eq!(response.status(), Status::BadRequest);
		assert_eq!(parse_err_response(response), Error::AuthDataEmpty);

		// CASE 2: tg auth not set
		registration_data.auth_data.tg_auth_token = None;
		registration_data.notifier = Notifier::Telegram("@dummy".to_string());

		let response = register(&client, &registration_data);

		assert_eq!(response.status(), Status::BadRequest);
		assert_eq!(parse_err_response(response), Error::AuthDataEmpty);
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
