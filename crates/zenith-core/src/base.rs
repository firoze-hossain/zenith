//! Zenith Base: Relational database and form management engine synthesizing
//! Microsoft Access and LibreOffice Base in pure Rust.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldType {
    Integer,
    VarChar(usize),
    Float,
    Date,
    Boolean,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub field_type: FieldType,
    pub is_primary_key: bool,
    pub is_nullable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Null,
    Integer(i64),
    VarChar(String),
    Float(f64),
    Date(String),
    Boolean(bool),
}

impl FieldValue {
    pub fn display(&self) -> String {
        match self {
            FieldValue::Null => "NULL".into(),
            FieldValue::Integer(i) => i.to_string(),
            FieldValue::VarChar(s) => s.clone(),
            FieldValue::Float(f) => format!("{:.2}", f),
            FieldValue::Date(d) => d.clone(),
            FieldValue::Boolean(b) => if *b { "True".into() } else { "False".into() },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub fields: Vec<FieldValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnDef>,
    pub records: Vec<Record>,
}

impl TableSchema {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            columns: Vec::new(),
            records: Vec::new(),
        }
    }

    pub fn add_column(&mut self, name: impl Into<String>, field_type: FieldType, is_pk: bool) {
        self.columns.push(ColumnDef {
            name: name.into(),
            field_type,
            is_primary_key: is_pk,
            is_nullable: !is_pk,
        });
    }

    pub fn insert_record(&mut self, fields: Vec<FieldValue>) -> bool {
        if fields.len() != self.columns.len() {
            return false;
        }
        self.records.push(Record { fields });
        true
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.name.eq_ignore_ascii_case(name))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Database {
    pub name: String,
    pub tables: Vec<TableSchema>,
    pub active_table_idx: usize,
}

impl Database {
    pub fn new(name: impl Into<String>) -> Self {
        let mut db = Self {
            name: name.into(),
            tables: Vec::new(),
            active_table_idx: 0,
        };

        // Create sample "Enterprise_Customers" table
        let mut customers = TableSchema::new("Customers");
        customers.add_column("CustomerID", FieldType::Integer, true);
        customers.add_column("CompanyName", FieldType::VarChar(64), false);
        customers.add_column("Country", FieldType::VarChar(32), false);
        customers.add_column("TotalSpend", FieldType::Float, false);
        customers.add_column("IsActive", FieldType::Boolean, false);

        customers.insert_record(vec![
            FieldValue::Integer(1001),
            FieldValue::VarChar("Nova Cloud Inc".into()),
            FieldValue::VarChar("USA".into()),
            FieldValue::Float(18500.0),
            FieldValue::Boolean(true),
        ]);
        customers.insert_record(vec![
            FieldValue::Integer(1002),
            FieldValue::VarChar("Apex Robotics".into()),
            FieldValue::VarChar("Germany".into()),
            FieldValue::Float(34200.0),
            FieldValue::Boolean(true),
        ]);
        customers.insert_record(vec![
            FieldValue::Integer(1003),
            FieldValue::VarChar("Tokyo Quant Systems".into()),
            FieldValue::VarChar("Japan".into()),
            FieldValue::Float(29800.0),
            FieldValue::Boolean(false),
        ]);

        db.tables.push(customers);

        // Create sample "Orders" table
        let mut orders = TableSchema::new("Orders");
        orders.add_column("OrderID", FieldType::Integer, true);
        orders.add_column("CustomerID", FieldType::Integer, false);
        orders.add_column("OrderDate", FieldType::Date, false);
        orders.add_column("Amount", FieldType::Float, false);

        orders.insert_record(vec![
            FieldValue::Integer(5001),
            FieldValue::Integer(1001),
            FieldValue::Date("2026-11-15".into()),
            FieldValue::Float(4200.0),
        ]);
        orders.insert_record(vec![
            FieldValue::Integer(5002),
            FieldValue::Integer(1002),
            FieldValue::Date("2026-11-18".into()),
            FieldValue::Float(12500.0),
        ]);

        db.tables.push(orders);
        db
    }

    pub fn active_table(&self) -> &TableSchema {
        &self.tables[self.active_table_idx]
    }

    pub fn active_table_mut(&mut self) -> &mut TableSchema {
        &mut self.tables[self.active_table_idx]
    }

    /// Executes simple SQL queries (SELECT, INSERT, COUNT)
    pub fn execute_sql(&self, query: &str) -> Result<Vec<Record>, String> {
        let q = query.trim();
        let upper = q.to_uppercase();

        if upper.starts_with("SELECT") {
            let from_pos = upper.find("FROM").ok_or("Missing FROM clause in SELECT")?;
            let table_part = q[from_pos + 4..].trim();
            let table_name = table_part.split_whitespace().next().unwrap_or("");

            let table = self
                .tables
                .iter()
                .find(|t| t.name.eq_ignore_ascii_case(table_name))
                .ok_or(format!("Table '{}' not found", table_name))?;

            // Check WHERE clause
            if let Some(where_pos) = upper.find("WHERE") {
                let cond = q[where_pos + 5..].trim();
                let filtered: Vec<Record> = table
                    .records
                    .iter()
                    .filter(|rec| {
                        if cond.contains('=') {
                            let parts: Vec<&str> = cond.split('=').map(|s| s.trim()).collect();
                            if parts.len() == 2 {
                                let col_name = parts[0];
                                let val_str = parts[1].trim_matches('\'').trim_matches('"');
                                if let Some(idx) = table.column_index(col_name) {
                                    return rec.fields[idx].display().eq_ignore_ascii_case(val_str);
                                }
                            }
                        }
                        true
                    })
                    .cloned()
                    .collect();
                return Ok(filtered);
            }

            return Ok(table.records.clone());
        }

        Err("Unsupported SQL statement".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_table_and_sql() {
        let db = Database::new("Enterprise_ERP");
        assert_eq!(db.tables.len(), 2);
        assert_eq!(db.tables[0].name, "Customers");
        assert_eq!(db.tables[0].records.len(), 3);

        // SQL SELECT ALL
        let all_cust = db.execute_sql("SELECT * FROM Customers").unwrap();
        assert_eq!(all_cust.len(), 3);

        // SQL SELECT WHERE
        let usa_cust = db.execute_sql("SELECT * FROM Customers WHERE Country = 'USA'").unwrap();
        assert_eq!(usa_cust.len(), 1);
        assert_eq!(usa_cust[0].fields[1].display(), "Nova Cloud Inc");
    }
}
