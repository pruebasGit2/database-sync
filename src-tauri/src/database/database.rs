use tiberius::{AuthMethod, Client, Config, Query, Row};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use crate::utils::{cstr_utils::Cstring, error_utils::DbError};

pub struct Database {
    pub name: String
}

impl Database {

    pub async fn get_all(cstr: &str) -> Result<Vec<Database>, DbError> {
        let cstring = Cstring::new(cstr)?;

        let mut config = Config::new();
        config.host(cstring.server);
        config.trust_cert();
        config.authentication(AuthMethod::sql_server(cstring.user, cstring.password));

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let mut client = Client::connect(config, tcp.compat_write()).await?;

        let query = Query::new("SELECT name FROM sys.databases;");

        let stream = query.query(&mut client).await?;
        let rows = stream.into_first_result().await?;

        let mut databases = Vec::new();

        for row in rows {
            databases.push(Database::from_row(&row)?);
        }

        client.close().await?;

        Ok(databases)
    }
    
}

impl CollectDb for Database {
    fn from_row(row: &Row) -> Result<Self, DbError> where Self: Sized {
        let name = row.get_row("name")?.unwrap();

        Ok(Database {
            name
        })
    }
}

pub struct Esquema {
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub column_default: Option<String>,
    pub is_nullable: String,
    pub data_type: String,
    pub character_maximum_length: Option<i32>,
    pub constraint_type: Option<String>,
    pub referenced_table_name: Option<String>,
    pub referenced_column_name: Option<String>
}

impl Esquema {
    
    pub async fn get_all(_: &str, database: &str) -> Result<Vec<Esquema>, DbError> {
        let mut config = Config::new();
        config.trust_cert();
        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;
        let mut client = Client::connect(config, tcp.compat_write()).await?;

        let sql = format!(r#"
USE [{}]

SELECT 
    C.TABLE_SCHEMA, 
    C.TABLE_NAME, 
    C.COLUMN_NAME, 
    C.COLUMN_DEFAULT, 
    C.IS_NULLABLE, 
    C.DATA_TYPE, 
    C.CHARACTER_MAXIMUM_LENGTH,
    TC.CONSTRAINT_TYPE,
    KCU2.TABLE_NAME AS REFERENCED_TABLE_NAME,
    KCU2.COLUMN_NAME AS REFERENCED_COLUMN_NAME
FROM 
    INFORMATION_SCHEMA.COLUMNS C
LEFT JOIN 
    INFORMATION_SCHEMA.KEY_COLUMN_USAGE KCU
    ON C.TABLE_NAME = KCU.TABLE_NAME 
    AND C.COLUMN_NAME = KCU.COLUMN_NAME 
    AND C.TABLE_SCHEMA = KCU.TABLE_SCHEMA
LEFT JOIN 
    INFORMATION_SCHEMA.TABLE_CONSTRAINTS TC
    ON KCU.CONSTRAINT_NAME = TC.CONSTRAINT_NAME 
    AND KCU.TABLE_SCHEMA = TC.TABLE_SCHEMA 
    AND KCU.TABLE_NAME = TC.TABLE_NAME
LEFT JOIN 
    INFORMATION_SCHEMA.REFERENTIAL_CONSTRAINTS AS RC
    ON TC.CONSTRAINT_NAME = RC.CONSTRAINT_NAME
LEFT JOIN 
    INFORMATION_SCHEMA.KEY_COLUMN_USAGE AS KCU2
    ON rc.UNIQUE_CONSTRAINT_NAME = kcu2.CONSTRAINT_NAME
    AND kcu.ORDINAL_POSITION = kcu2.ORDINAL_POSITION
ORDER BY 
    C.TABLE_SCHEMA, 
    C.TABLE_NAME,
    C.ORDINAL_POSITION;"#, database
        );

        let query = Query::new(sql);

        let stream = query.query(&mut client).await?;
        let rows = stream.into_first_result().await?;

        let mut esquemas = Vec::new();

        for row in rows {
            esquemas.push(Esquema::from_row(&row)?);
        }

        client.close().await?;

        Ok(esquemas)
    }

}

impl CollectDb for Esquema {
    fn from_row(row: &Row) -> Result<Self, DbError> {
        let table_schema = row.get_row("TABLE_SCHEMA")?.unwrap();
        let table_name = row.get_row("TABLE_NAME")?.unwrap();
        let column_name = row.get_row("COLUMN_NAME")?.unwrap();
        let column_default = row.get_row("COLUMN_DEFAULT")?;
        let is_nullable = row.get_row("IS_NULLABLE")?.unwrap();
        let data_type = row.get_row("DATA_TYPE")?.unwrap();
        let character_maximum_length = row.get_row_number("CHARACTER_MAXIMUM_LENGTH")?;
        let constraint_type = row.get_row("CONSTRAINT_TYPE")?;
        let referenced_table_name = row.get_row("REFERENCED_TABLE_NAME")?;
        let referenced_column_name = row.get_row("REFERENCED_COLUMN_NAME")?;

        Ok(Esquema {
            table_schema,
            table_name,
            column_name,
            column_default,
            is_nullable,
            data_type,
            character_maximum_length,
            constraint_type,
            referenced_table_name,
            referenced_column_name
        })
    }
}

pub trait CollectDb {
    fn from_row(row: &Row) -> Result<Self, DbError> where Self: Sized;
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