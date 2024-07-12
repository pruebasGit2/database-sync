use std::collections::HashMap;
use itertools::Itertools;

use crate::database::esquema::Esquema;

pub struct Db {
    pub name: String,
    pub tables: HashMap<String, Table>
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
            _tables.insert(table.name.clone(), table);
        }

        Db { name, tables: _tables }
    }

    pub fn compare(&mut self, other_db: &mut Db) -> Vec<String> {
        
        let mut scripts: Vec<String> = Vec::new();

        for table in self.tables.values_mut() {
            
            if let Some(other_table) = other_db.tables.get_mut(&table.name) {
                for column in table.columns.values_mut() {
                    if let Some(_other_column) = other_table.columns.get_mut(&column.name) {

                    } else {
                        scripts.push(column.alter_add(table.database.clone()));
                    }
                }
            } else {
                scripts.push(table.script());
            }

        }

        scripts
    }

}

pub struct Table {
    pub name: String,
    pub database: String,
    pub columns: HashMap<String, Column>
}

impl Table {

    pub fn new(database: String, columns: Vec<Esquema>) -> Self {

        let name = if let Some(first) = columns.first() {
            first.table_name.clone()
        } else { String::new() };

        let mut _columns: HashMap<String, Column> = HashMap::new();

        for column in columns {
            _columns.insert(column.column_name.clone(), Column::new(column));
        }

        Table { name, database, columns: _columns }
    }

    pub fn script(&mut self) -> String
    {
        let mut columns_created = Vec::new();

        for column in self.columns.values_mut() {
            columns_created.push(format!("{}", column.script(false)));
        }

        let columns = columns_created.join(",\r\n");

        format!(r#"
CREATE TABLE {}(
    {}
)
GO
"#, self.database, columns)
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
    pub referenced_column: Option<String>
    
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
            referenced_column: schema.referenced_column_name
        }
    }

    pub fn script(&mut self, default_value: bool) -> String {
        let numbers = vec!["bigint", "bit", "float", "int", "smallint", "tinyint"];
        let strings = vec!["nchar", "nvarchar", "text", "varchar"];

        if default_value && self.is_nullable == "NO" && self.column_default == None
        {
            if numbers.contains(&self.data_type.as_ref()) {
                self.column_default = Some(String::from("((0))"));
            } else if strings.contains(&self.data_type.as_ref()) {
                self.column_default = Some(String::from("((''))"));
            }
        }

        let items = vec![
            self.get_name(), self.get_type(), self.get_len(), self.get_is_null(), self.get_default(), self.get_constraint()
        ];

        items.iter().filter(|item| *item != "").join(" ")
    }

    pub  fn alter_add(&mut self, database: String) -> String
    {
        format!("ALTER TABLE {} ADD {}", database, self.script(true))
    }


    fn get_name(&self) -> String { 
        format!("[{}]", self.name)
    }
    
    fn get_type(&self) -> String { 
        format!("[{}]", self.data_type)
    }
    
    fn get_len(&self) -> String { 
        if self.data_type.clone() == "text" {
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
            _ => String::from("NOT NULL")
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
                if contraint == "FOREIGN KEY" && self.referenced_table.is_some() && self.referenced_column.is_some() {
                    format!("{} REFERENCES {}({})", contraint, self.referenced_table.clone().unwrap(), self.referenced_column.clone().unwrap())
                } else {
                    contraint
                }
            },
            None => String::new(),
        }
    }
    

}

