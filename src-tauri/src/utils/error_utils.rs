#[derive(Debug)]
pub enum DbError {
    Db(tiberius::error::Error),
    Io(std::io::Error),
    Cstr(connection_string::Error)
}

impl From<tiberius::error::Error> for DbError {
    fn from(value: tiberius::error::Error) -> Self {
        Self::Db(value)
    }
}

impl From<std::io::Error> for DbError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<connection_string::Error> for DbError {
    fn from(value: connection_string::Error) -> Self {
        Self::Cstr(value)
    }
}

impl Into<String> for DbError {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Db(e) => write!(f, "ParseIntError: {}", e.to_string()),
            Self::Io(e) => write!(f, "ParseFloatError: {}", e.to_string()),
            Self::Cstr(e) => write!(f, "Invalid connection string: {}", e.to_string()),
        }
    }
}