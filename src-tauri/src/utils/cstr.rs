use connection_string::AdoNetString;

use super::error_utils::DbError;

pub trait CstrGet {
    fn get_value(&self, val: &str) -> Result<String, DbError>;
}

impl CstrGet for AdoNetString {
    fn get_value(&self, val: &str) -> Result<String, DbError> {
        match self.get(val) {
            Some(v) => Ok(v.to_owned()),
            None => Err(DbError::Cstr(connection_string::Error::new(format!("Argument not found '{}'", val).as_str()))),
        }
    }
}
