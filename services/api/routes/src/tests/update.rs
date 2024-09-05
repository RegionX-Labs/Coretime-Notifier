use rocket::{
	http::{ContentType, Status},
	local::blocking::{Client, LocalResponse},
	routes,
};
use storage::{init_db, users::User};
use types::Notifier;

use crate::{
	query::user,
	register::{register_user, RegistrationData},
	tests::mock::parse_ok_response,
	update::{update_user, UpdateData},
	LOG_TARGET,
};

use super::mock::execute_with;

pub const DB_PATH: &'static str = "update-tests.db";

#[test]
fn updating_users_works() {
	execute_with(DB_PATH, || {
		let conn = init_db(DB_PATH).unwrap();
		let rocket = rocket::build()
			.manage(conn)
			.mount("/", routes![register_user, user, update_user]);
		let client = Client::tracked(rocket).expect("failed to create client");

		let registration_data = RegistrationData {
			id: 0,
			notifier: Notifier::Telegram,
			email: None,
			tg_handle: Some("@dummy".to_string()),
			enabled_notifications: vec![],
		};

		// Should register successfully
		let response = register(&client, &registration_data);
		assert_eq!(response.status(), Status::Ok);

		let response = client.get("/user/0").dispatch();
		assert_eq!(
			parse_ok_response(response),
			User {
				id: 0,
				email: None,
				tg_handle: Some("@dummy".to_string()),
				notifier: Notifier::Telegram,
			}
		);

		// Update to mismatched notifier should not work
		let mut update_data = UpdateData {
			id: 0,
			email: None,
			tg_handle: Some("@dummy".to_string()),
			notifier: Some(Notifier::Email),
		};
		let response = update(&client, &update_data);
		assert_eq!(response.status(), Status::BadRequest);

		// Update email with notifier works
		let update_data = UpdateData {
			id: 0,
			email: Some("dummy@mail.com".to_string()),
			tg_handle: Some("@dummy".to_string()),
			notifier: Some(Notifier::Email),
		};
		let response = update(&client, &update_data);
		assert_eq!(response.status(), Status::Ok);

		// Should return the updated user information
		let response = client.get("/user/0").dispatch();
		assert_eq!(
			parse_ok_response(response),
			User {
				id: 0,
				email: Some("dummy@mail.com".to_string()),
				tg_handle: Some("@dummy".to_string()),
				notifier: Notifier::Email,
			}
		);
	})
}

fn register<'a>(client: &'a Client, data: &'a RegistrationData) -> LocalResponse<'a> {
	client
		.post("/register_user")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&data).unwrap())
		.dispatch()
}

fn update<'a>(client: &'a Client, data: &'a UpdateData) -> LocalResponse<'a> {
	client
		.put("/update_user")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&data).unwrap())
		.dispatch()
}
