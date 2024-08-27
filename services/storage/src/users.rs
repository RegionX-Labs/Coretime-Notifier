use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use types::Notifier;

/// The data stored for each user in the database.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
	/// A unique identifier for a user.
	pub id: u32,
	/// Defines the channel through which the user would like to be notified.
	pub notifier: Notifier,
}

impl User {
	pub fn query_all(conn: &Connection) -> Result<Vec<User>> {
		let mut stmt = conn.prepare("SELECT * FROM users")?;
		let users_iter = stmt.query_map((), |row| {
			let email = row.get("email")?;
			let tg_handle = row.get("tg_handle")?;

			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email(email),
				"telegram" => Notifier::Telegram(tg_handle),
				_ => Notifier::Null,
			};

			Ok(User { id: row.get("id")?, notifier })
		})?;

		let users = users_iter.filter_map(Result::ok).collect();
		Ok(users)
	}

	pub fn query_by_id(conn: &Connection, id: u32) -> Result<Option<User>> {
		let mut smth = conn.prepare("SELECT * FROM users WHERE id=?1")?;
		let mut users_iter = smth.query_map(&[&id], |row| {
			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email(row.get("email")?),
				"telegram" => Notifier::Telegram(row.get("tg_handle")?),
				_ => Notifier::Null,
			};

			Ok(User { id, notifier })
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
				"email" => Notifier::Email(row.get("email")?),
				"telegram" => Notifier::Telegram(row.get("tg_handle")?),
				_ => Notifier::Null,
			};
			Ok(User { id: row.get("id")?, notifier })
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
				"email" => Notifier::Email(row.get("email")?),
				"telegram" => Notifier::Telegram(row.get("tg_handle")?),
				_ => Notifier::Null,
			};
			Ok(User { id: row.get("id")?, notifier })
		})?;

		match users_iter.next() {
			Some(Ok(data)) => Ok(Some(data)),
			Some(Err(err)) => Err(err),
			None => Ok(None),
		}
	}

	pub fn create_user(conn: &Connection, user: &User) -> Result<()> {
		let email = match &user.notifier {
			Notifier::Email(e) => Some(e),
			_ => None,
		};

		let tg_handle = match &user.notifier {
			Notifier::Telegram(t) => Some(t),
			_ => None,
		};

		let notifier = match user.notifier {
			Notifier::Email(_) => Some("email"),
			Notifier::Telegram(_) => Some("telegram"),
			Notifier::Null => None,
		};

		match notifier {
			Some(notifier) => {
				conn.execute(
					"INSERT INTO users
                        (id, email, tg_handle, notifier)
                        VALUES (?1, ?2, ?3, ?4)
                    ",
					params![user.id, email, tg_handle, notifier],
				)?;
			},
			None => {
				conn.execute(
					"INSERT INTO users
                        (email, tg_handle, notifier)
                        VALUES (?1, ?2, ?3, NULL)
                    ",
					params![user.id, email, tg_handle],
				)?;
			},
		};
		Ok(())
	}
}
