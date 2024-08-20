// #[post("/user/<user_id>")]
// pub async fn user(
// 	user_id: u32,
// ) -> Result<status::Custom<()>, status::Custom<Json<ErrorResponse>>> {
// 	let conn = &User::get_connection().expect("DB connection not established");

// 	registration_data.validate().map_err(|error| {
// 		status::Custom(Status::BadRequest, Json(ErrorResponse { message: error.to_string() }))
// 	})?;

//     Ok(())
// }
