use crate::{
	query::user,
	register::{register_user, RegistrationData},
	tests::mock::{execute_with, DB_PATH},
};
use rocket::{
	http::{ContentType, Status},
	local::blocking::{Client, LocalResponse},
	routes,
};
use serde_json::from_str;
use storage::{init_db, users::User};
use types::{api::ErrorResponse, Notifier};

#[test]
#[should_panic]
fn no_db_connection_errors() {
	// Should panic because no db connection provided.
	let rocket = rocket::build().mount("/", routes![register_user, user]);
	let _client = Client::tracked(rocket).expect("failed to create a client");
}

#[test]
fn register_works() {
	execute_with(|| {
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
		// CASE 1: the user has not set a notifier.
		let response = client
			.post("/register_user")
			.header(ContentType::JSON)
			.body(serde_json::to_string(&registration_data).unwrap())
			.dispatch();

		assert_eq!(response.status(), Status::BadRequest);
		assert_eq!(parse_err_response(response), "Email must not be empty".into());

		// CASE 2: correct data, should work.
		registration_data.email = Some("dummy@gmail.com".to_string());
		let response = client
			.post("/register_user")
			.header(ContentType::JSON)
			.body(serde_json::to_string(&registration_data).unwrap())
			.dispatch();
		assert_eq!(response.status(), Status::Ok);

		let response = client.get("/user/0").dispatch();
		// After registering we should be able to get the user:
		assert_eq!(
			parse_ok_response(response),
			User {
				id: 0,
				email: Some("dummy@gmail.com".to_string()),
				tg_handle: None,
				notifier: Notifier::Email,
			}
		);
	});
}

fn parse_ok_response<'a>(response: LocalResponse<'a>) -> User {
	let body = response.into_string().unwrap();
	serde_json::from_str(&body).expect("can't parse value")
}

fn parse_err_response<'a>(response: LocalResponse<'a>) -> ErrorResponse {
	let body = response.into_string().unwrap();
	from_str(&body).unwrap()
}
