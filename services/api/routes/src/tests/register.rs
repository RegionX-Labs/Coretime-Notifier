use crate::register::{register_user, RegistrationData};
use rocket::{
	http::{ContentType, Status},
	local::blocking::Client,
	routes,
};
use storage::init_db;
use types::Notifier;

#[test]
fn register_works() {
	// We reset the db before each test. <- TODO
	// Don't check the result since it will error if the db already doesn't exist which isn't an
	// issue.
	let _ = std::fs::remove_file("test.db");
	let rocket = rocket::build()
		.manage(init_db("test.db").unwrap())
		.mount("/", routes![register_user]);

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
