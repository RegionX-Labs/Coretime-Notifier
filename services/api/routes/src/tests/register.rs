use crate::{
	errors::Error,
	query::user,
	register::{register_user, RegistrationData},
	tests::mock::{execute_with, parse_err_response, parse_ok_response},
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

		let mut registration_data = RegistrationData {
			id: 0,
			notifier: Notifier::Email,
			email: None,
			tg_handle: None,
			enabled_notifications: vec![],
		};
		// CASE 1: the user did not set the notifier.
		let response = register(&client, &registration_data);

		assert_eq!(response.status(), Status::BadRequest);
		assert_eq!(parse_err_response(response), Error::NotifierEmpty);

		// CASE 2: correct data, should work.
		registration_data.email = Some("dummy@gmail.com".to_string());
		registration_data.tg_handle = Some("@dummy".to_string());

		let response = register(&client, &registration_data);
		assert_eq!(response.status(), Status::Ok);

		let response = client.get("/user/0").dispatch();
		// After registering we should be able to get the user:
		assert_eq!(
			parse_ok_response(response),
			User {
				id: 0,
				email: Some("dummy@gmail.com".to_string()),
				tg_handle: Some("@dummy".to_string()),
				notifier: Notifier::Email,
			}
		);

		// CASE 3: user with the same id exists
		let response = register(&client, &registration_data);
		assert_eq!(response.status(), Status::Conflict);
		assert_eq!(parse_err_response(response), Error::UserExists);

		// CASE 4: user with the same email exists:
		let registration_data = RegistrationData {
			id: 1,
			notifier: Notifier::Email,
			email: Some("dummy@gmail.com".to_string()),
			tg_handle: None,
			enabled_notifications: vec![],
		};
		let response = register(&client, &registration_data);

		assert_eq!(response.status(), Status::Conflict);
		assert_eq!(parse_err_response(response), Error::NotifierNotUnique);

		// CASE 5: user with the same telegram exists:
		let registration_data = RegistrationData {
			id: 1,
			notifier: Notifier::Telegram,
			email: None,
			tg_handle: Some("@dummy".to_string()),
			enabled_notifications: vec![],
		};

		let response = register(&client, &registration_data);
		assert_eq!(response.status(), Status::Conflict);
		assert_eq!(parse_err_response(response), Error::NotifierNotUnique);
	});
}

fn register<'a>(client: &'a Client, data: &'a RegistrationData) -> LocalResponse<'a> {
	client
		.post("/register_user")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&data).unwrap())
		.dispatch()
}
