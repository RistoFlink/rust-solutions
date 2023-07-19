use std::fs;

use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for _value in contents.lines() {
            rows.push(Row::from("Hello, world"));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
        })
    }
    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
