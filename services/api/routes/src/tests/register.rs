use crate::{
	query::user,
	register::{register_user, RegistrationData},
};
use rocket::{
	http::{ContentType, Status},
	local::blocking::{Client, LocalResponse},
	routes,
};
use storage::{init_db, users::User};
use types::Notifier;

#[test]
fn register_works() {
	// We reset the db before each test. <- TODO
	// Don't check the result since it will error if the db already doesn't exist which isn't an
	// issue.
	let _ = std::fs::remove_file("test.db");
	let conn = init_db("test.db").unwrap();
	let rocket = rocket::build().manage(conn).mount("/", routes![register_user, user]);

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

	let response = client.get("/user/0").dispatch();
	assert_eq!(response.status(), Status::Ok);

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
}

fn parse_ok_response<'a>(response: LocalResponse<'a>) -> User {
	let body = response.into_string().unwrap();
	serde_json::from_str(&body).expect("can't parse value")
}
