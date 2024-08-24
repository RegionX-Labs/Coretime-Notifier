pub mod query;
pub mod register;
pub mod update;

mod errors;

#[cfg(test)]
mod tests;

pub const LOG_TARGET: &str = "api-routes";
