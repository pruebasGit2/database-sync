use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::database::esquema::Esquema;

pub struct Db {
    pub name: String,
    pub tables: HashMap<String, Table>,
}

impl Db {
    pub fn new(name: String, eschemas: Vec<Esquema>) -> Self {
        let mut _tables = HashMap::new();

        let tables: Vec<Table> = eschemas
            .into_iter()
            .chunk_by(|esq| esq.get_db())
            .into_iter()
            .map(|esq| Table::new(name.clone(), esq.1.collect_vec()))
            .collect();

        for table in tables {
            _tables.insert(table.get_ref(), table);
        }

        Db {
            name,
            tables: _tables,
        }
    }

    pub fn compare(&mut self, other_db: &mut Db) -> Vec<String> {
        let mut not_pepe = HashMap::<&str, &str>::new();

        not_pepe.insert("VARBINARY", "VARCHAR");
        not_pepe.insert("VARBINARY", "CHAR");
        not_pepe.insert("BINARY", "VARCHAR");
        not_pepe.insert("BINARY", "CHAR");
        not_pepe.insert("VARCHAR", "VARBINARY");
        not_pepe.insert("CHAR", "VARBINARY");
        not_pepe.insert("TEXT", "VARBINARY");
        not_pepe.insert("NTEXT", "VARBINARY");
        not_pepe.insert("IMAGE", "VARCHAR");
        not_pepe.insert("IMAGE", "CHAR");
        not_pepe.insert("XML", "VARCHAR");
        not_pepe.insert("XML", "CHAR");
        not_pepe.insert("INT", "VARCHAR");
        not_pepe.insert("INT", "CHAR");
        not_pepe.insert("BIGINT", "VARCHAR");
        not_pepe.insert("BIGINT", "CHAR");
        not_pepe.insert("SMALLINT", "VARCHAR");
        not_pepe.insert("SMALLINT", "CHAR");
        not_pepe.insert("TINYINT", "VARCHAR");
        not_pepe.insert("TINYINT", "CHAR");
        not_pepe.insert("FLOAT", "VARCHAR");
        not_pepe.insert("FLOAT", "CHAR");
        not_pepe.insert("DECIMAL", "VARCHAR");
        not_pepe.insert("DECIMAL", "CHAR");
        not_pepe.insert("NUMERIC", "VARCHAR");
        not_pepe.insert("NUMERIC", "CHAR");
        not_pepe.insert("DATE", "VARCHAR");
        not_pepe.insert("DATE", "CHAR");
        not_pepe.insert("DATETIME", "VARCHAR");
        not_pepe.insert("DATETIME", "CHAR");
        not_pepe.insert("DATETIME2", "VARCHAR");
        not_pepe.insert("DATETIME2", "CHAR");
        not_pepe.insert("TIME", "VARCHAR");
        not_pepe.insert("TIME", "CHAR");
        not_pepe.insert("SMALLDATETIME", "VARCHAR");
        not_pepe.insert("SMALLDATETIME", "CHAR");
        not_pepe.insert("TIMESTAMP", "VARCHAR");
        not_pepe.insert("TIMESTAMP", "CHAR");
        not_pepe.insert("UNIQUEIDENTIFIER", "VARCHAR");
        not_pepe.insert("UNIQUEIDENTIFIER", "CHAR");

        let mut scripts: Vec<String> = Vec::new();

        // iterate over database tables
        for table in self.tables.values_mut() {
            // check if other database has table
            if let Some(other_table) = other_db.tables.get_mut(&table.get_ref()) {
                let other_table_ref = other_table.get_full_ref();
                // iterate over other table columns
                for column in table.columns.values_mut() {
                    if let Some(other_column) = other_table.columns.get_mut(&column.name) {
                        // if column exist check the data type
                        if column.data_type != other_column.data_type {
                            if let Some(data_type) = not_pepe.get(column.data_type.as_str()) {
                                if other_column.data_type.eq(data_type) {
                                    continue;
                                }
                            }
                            
                            println!("{}.[{}] - {} >> {}", other_table_ref.clone(), other_column.name, other_column.data_type, column.data_type);
                            scripts.push(format!("ALTER TABLE {} ALTER COLUMN {} {}", other_table_ref.clone(), other_column.name, column.data_type));
                        }

                    } else {
                        scripts.push(column.alter_add(other_table_ref.clone()));
                    }
                }
            } else {
                // Create table if not exists in other database
                scripts.push(table.script(other_db.name.clone()));
            }
        }

        // Convert Vec<String> to HashSet<String> to remove duplicates
        let set: HashSet<String> = scripts.drain(..).collect();

        // Convert HashSet<String> back to Vec<String>
        let scripts: Vec<String> = set.into_iter().collect();

        scripts
    }
}

pub struct Table {
    pub name: String,
    pub schema: String,
    pub database: String,
    pub columns: HashMap<String, Column>,
}

impl Table {
    pub fn new(database: String, columns: Vec<Esquema>) -> Self {
        let mut name = String::new();
        let mut schema = String::new();

        if let Some(first) = columns.first() {
            name = first.table_name.clone();
            schema = first.table_schema.clone();
        }

        let mut _columns: HashMap<String, Column> = HashMap::new();

        for column in columns {
            _columns.insert(column.column_name.clone(), Column::new(column));
        }

        Table {
            name,
            schema,
            database,
            columns: _columns,
        }
    }

    pub fn script(&mut self, db_name: String) -> String {
        let mut columns_created = Vec::new();

        for column in self.columns.values_mut() {
            columns_created.push(format!("{}", column.script(false)));
        }

        let columns = columns_created.join(",\r\n\t");

        format!(
            r#"
CREATE TABLE [{}].{}(
{}
)
GO"#,
            db_name,
            self.get_ref(),
            format!("\t{}", columns)
        )
    }

    pub fn get_ref(&self) -> String {
        format!("[{}].[{}]", self.schema, self.name)
    }

    pub fn get_full_ref(&self) -> String {
        format!("[{}].[{}].[{}]", self.database, self.schema, self.name)
    }
}

pub struct Column {
    pub name: String,
    pub data_type: String,
    pub character_maximum_length: Option<i32>,
    pub is_nullable: String,
    pub column_default: Option<String>,
    pub constrain_type: Option<String>,
    pub referenced_table: Option<String>,
    pub referenced_column: Option<String>,
}

impl Column {
    pub fn new(schema: Esquema) -> Self {
        Column {
            name: schema.column_name,
            data_type: schema.data_type,
            character_maximum_length: schema.character_maximum_length,
            is_nullable: schema.is_nullable,
            column_default: schema.column_default,
            constrain_type: schema.constraint_type,
            referenced_table: schema.referenced_table_name,
            referenced_column: schema.referenced_column_name,
        }
    }

    pub fn script(&mut self, default_value: bool) -> String {
        let numbers = vec!["bigint", "bit", "float", "int", "smallint", "tinyint"];
        let strings = vec!["nchar", "nvarchar", "text", "varchar"];

        if default_value && self.is_nullable == "NO" && self.column_default == None {
            if numbers.contains(&self.data_type.as_ref()) {
                self.column_default = Some(String::from("((0))"));
            } else if strings.contains(&self.data_type.as_ref()) {
                self.column_default = Some(String::from("((''))"));
            }
        }

        let items = vec![
            self.get_name(),
            self.get_type(),
            self.get_len(),
            self.get_is_null(),
            self.get_default(),
            self.get_constraint(),
        ];

        items.iter().filter(|item| *item != "").join(" ")
    }

    pub fn alter_add(&mut self, table: String) -> String {
        format!("ALTER TABLE {} ADD {}\r\nGO", table, self.script(true))
    }

    fn get_name(&self) -> String {
        format!("[{}]", self.name)
    }

    fn get_type(&self) -> String {
        format!("[{}]", self.data_type)
    }

    fn get_len(&self) -> String {
        if !self.data_type.eq("text") {
            if let Some(len) = self.character_maximum_length {
                if len == -1 {
                    String::from("(MAX)")
                } else {
                    format!("({})", len)
                }
            } else {
                String::from("")
            }
        } else {
            String::from("")
        }
    }

    fn get_is_null(&self) -> String {
        match self.is_nullable.clone().as_ref() {
            "YES" => String::from("NULL"),
            _ => String::from("NOT NULL"),
        }
    }

    fn get_default(&self) -> String {
        match self.column_default.clone() {
            Some(def) => format!("DEFAULT {}", def),
            None => String::new(),
        }
    }

    fn get_constraint(&self) -> String {
        match self.constrain_type.clone() {
            Some(contraint) => {
                if contraint == "FOREIGN KEY"
                    && self.referenced_table.is_some()
                    && self.referenced_column.is_some()
                {
                    format!(
                        "{} REFERENCES {}({})",
                        contraint,
                        self.referenced_table.clone().unwrap(),
                        self.referenced_column.clone().unwrap()
                    )
                } else {
                    contraint
                }
            }
            None => String::new(),
        }
    }
}
