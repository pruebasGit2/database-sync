pub enum DbError {
    Db(tiberius::error::Error),
    Io(std::io::Error),
    Parse(url::ParseError)
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

impl From<url::ParseError> for DbError {
    fn from(value: url::ParseError) -> Self {
        Self::Parse(value)
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
            Self::Parse(e) => write!(f, "Invalid connection string: {}", e.to_string())
        }
    }
}