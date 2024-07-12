use connection_string::AdoNetString;
use tiberius::{AuthMethod, Client, Config, Query, Row};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use crate::utils::{cstr::CstrGet, error_utils::DbError};

use super::row::{CollectDb, RowUtils};

pub struct DatabaseSql {
    pub name: String,
}

impl DatabaseSql {
    pub async fn get_all(cstr: &str) -> Result<Vec<DatabaseSql>, DbError> {
        let cstring: AdoNetString = cstr.parse()?;

        let mut config = Config::new();
        config.host(cstring.get_value("server")?);
        config.trust_cert();
        config.authentication(AuthMethod::sql_server(
            cstring.get_value("user")?,
            cstring.get_value("password")?,
        ));

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let mut client = Client::connect(config, tcp.compat_write()).await?;

        let query = Query::new("SELECT name FROM sys.databases ORDER BY name;");

        let stream = query.query(&mut client).await?;
        let rows = stream.into_first_result().await?;

        let mut databases = Vec::new();

        for row in rows {
            databases.push(DatabaseSql::from_row(&row)?);
        }

        client.close().await?;

        Ok(databases)
    }
}

impl CollectDb for DatabaseSql {
    fn from_row(row: &Row) -> Result<Self, DbError>
    where
        Self: Sized,
    {
        let name = row.get_row("name")?.unwrap();

        Ok(DatabaseSql { name })
    }
}