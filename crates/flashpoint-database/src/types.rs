use diesel::SqliteConnection;

#[derive(Debug)]
pub enum ErrorKind {
  DbOpenFailed(String),
  DbReadFailed,
}

/// The data required for database state initialization.
#[derive(Debug)]
pub struct InitData {
  pub db_path: String,
}

/// An opaque structure that holds the current database state.
pub struct DbState {
  pub(crate) conn: SqliteConnection,
}
