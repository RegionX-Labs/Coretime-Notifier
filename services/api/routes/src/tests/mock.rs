pub fn execute_with<R>(db_path: &str, f: impl Fn() -> R) -> R {
	// Don't check the result since it will error if the db already doesn't exist which isn't an
	// issue.
	let _ = std::fs::remove_file(db_path);

	f()
}
