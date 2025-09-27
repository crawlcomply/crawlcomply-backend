#[cfg(test)]
pub static INIT_DB: std::sync::Once = std::sync::Once::new();

#[cfg(test)]
mod profile;

#[cfg(test)]
mod org;

#[cfg(test)]
mod repo;

#[cfg(test)]
mod repo_history;

#[cfg(test)]
mod run_history;

mod helpers;
