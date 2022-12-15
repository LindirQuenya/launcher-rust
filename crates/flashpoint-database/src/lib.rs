use diesel::prelude::*;

pub mod models;
pub mod schema;
pub mod types;
use models::*;
use types::ErrorKind::{DbOpenFailed, DbReadFailed};
use types::{DbState, InitData};

fn establish_connection(db_path: &str) -> Result<SqliteConnection, types::ErrorKind> {
  match SqliteConnection::establish(db_path) {
    Ok(conn) => Ok(conn),
    Err(_) => Err(DbOpenFailed(db_path.to_owned())),
  }
}

pub fn get_all_games(state: &mut DbState) -> Result<Vec<Game>, types::ErrorKind> {
  get_games(state, None)
}

pub fn get_games(state: &mut DbState, limit: Option<i64>) -> Result<Vec<Game>, types::ErrorKind> {
  use self::schema::game::dsl::*;

  let loadresult = match limit {
    Some(lim) => game.limit(lim).load::<Game>(&mut state.conn),
    None => game.load::<Game>(&mut state.conn),
  };

  match loadresult {
    Ok(games) => Ok(games),
    Err(_) => Err(DbReadFailed),
  }
}

pub fn initialize(data: InitData) -> Result<DbState, types::ErrorKind> {
  let conn = establish_connection(&data.db_path)?;
  Ok(DbState { conn })
}
