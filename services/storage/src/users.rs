use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use types::Notifier;

/// The data stored for each user in the database.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
	/// A unique identifier for a user.
	pub id: u32,
	/// Email of the user.
	pub email: Option<String>,
	/// Telegram handle of the user.
	pub tg_handle: Option<String>,
	/// Defines the channel through which the user would like to be notified.
	pub notifier: Notifier,
}

impl User {
	pub fn query_all(conn: &Connection) -> Result<Vec<User>> {
		let mut stmt = conn.prepare("SELECT * FROM users")?;
		let users_iter = stmt.query_map((), |row| {
			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email,
				"telegram" => Notifier::Telegram,
				_ => Notifier::Null,
			};

			Ok(User {
				id: row.get("id")?,
				tg_handle: row.get("tg_handle")?,
				email: row.get("email")?,
				notifier,
			})
		})?;

		let users = users_iter.filter_map(Result::ok).collect();
		Ok(users)
	}

	pub fn query_by_id(conn: &Connection, id: u32) -> Result<Option<User>> {
		let mut smth = conn.prepare("SELECT * FROM users WHERE id=?1")?;
		let mut users_iter = smth.query_map(&[&id], |row| {
			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email,
				"telegram" => Notifier::Telegram,
				_ => Notifier::Null,
			};
			Ok(User {
				id: row.get("id")?,
				tg_handle: row.get("tg_handle")?,
				email: row.get("email")?,
				notifier,
			})
		})?;

		match users_iter.next() {
			Some(Ok(data)) => Ok(Some(data)),
			Some(Err(err)) => Err(err),
			None => Ok(None),
		}
	}

	pub fn query_by_email(conn: &Connection, email: String) -> Result<Option<User>> {
		let mut smth = conn.prepare("SELECT * FROM users WHERE email=?1")?;
		let mut users_iter = smth.query_map(&[&email], |row| {
			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email,
				"telegram" => Notifier::Telegram,
				_ => Notifier::Null,
			};
			Ok(User {
				id: row.get("id")?,
				tg_handle: row.get("tg_handle")?,
				email: row.get("email")?,
				notifier,
			})
		})?;

		match users_iter.next() {
			Some(Ok(data)) => Ok(Some(data)),
			Some(Err(err)) => Err(err),
			None => Ok(None),
		}
	}

	pub fn query_by_tg_handle(conn: &Connection, handle: String) -> Result<Option<User>> {
		let mut smth = conn.prepare("SELECT * FROM users WHERE tg_handle=?1")?;
		let mut users_iter = smth.query_map(&[&handle], |row| {
			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email,
				"telegram" => Notifier::Telegram,
				_ => Notifier::Null,
			};
			Ok(User {
				id: row.get("id")?,
				tg_handle: row.get("tg_handle")?,
				email: row.get("email")?,
				notifier,
			})
		})?;

		match users_iter.next() {
			Some(Ok(data)) => Ok(Some(data)),
			Some(Err(err)) => Err(err),
			None => Ok(None),
		}
	}

	pub fn create_user(conn: &Connection, user: &User) -> Result<()> {
		let User { id, email, tg_handle, .. } = user;
		let notifier = Self::notifier_to_text(&user.notifier);

		match notifier {
			Some(notifier) => {
				conn.execute(
					"INSERT INTO users
                        (id, email, tg_handle, notifier)
                        VALUES (?1, ?2, ?3, ?4)
                    ",
					params![id, email, tg_handle, notifier],
				)?;
			},
			None => {
				conn.execute(
					"INSERT INTO users
                        (email, tg_handle, notifier)
                        VALUES (?1, ?2, ?3, NULL)
                    ",
					params![id, email, tg_handle],
				)?;
			},
		};
		Ok(())
	}

	pub fn update(conn: &Connection, user: &User) -> Result<()> {
		let User { id, email, tg_handle, .. } = user;
		let notifier = Self::notifier_to_text(&user.notifier);

		if notifier.is_some() {
			conn.execute(
				"UPDATE users SET
					email = ?1, tg_handle = ?2, notifier = ?3
					WHERE id = ?4
				", 
				params![email, tg_handle, notifier.unwrap(), id],
			)?;
		} else {
			conn.execute(
				"UPDATE users SET
					email = ?1, tg_handle = ?2, notifier = NULL
					WHERE id = ?3
				", 
				params![email, tg_handle, id],
			)?;
		}

		Ok(())
	}

	fn notifier_to_text(notifier: &Notifier) -> Option<String> {
		match notifier {
			Notifier::Email => Some(String::from("email")),
			Notifier::Telegram => Some("telegram".to_string()),
			_ => None,
		}
	}
}
