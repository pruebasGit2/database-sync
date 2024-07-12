use tiberius::Row;

use crate::utils::error_utils::DbError;

pub trait CollectDb {
    fn from_row(row: &Row) -> Result<Self, DbError>
    where
        Self: Sized;
}

pub trait RowUtils {
    fn get_row(&self, name: &str) -> Result<Option<String>, DbError>;
    fn get_row_number(&self, name: &str) -> Result<Option<i32>, DbError>;
}

impl RowUtils for Row {
    fn get_row(&self, name: &str) -> Result<Option<String>, DbError> {
        match self.try_get::<&str, _>(name) {
            Ok(res) => Ok(res.map(ToString::to_string)),
            Err(e) => Err(DbError::Db(e)),
        }
    }
    fn get_row_number(&self, name: &str) -> Result<Option<i32>, DbError> {
        match self.try_get::<i32, _>(name) {
            Ok(res) => Ok(res),
            Err(e) => Err(DbError::Db(e)),
        }
    }
}
