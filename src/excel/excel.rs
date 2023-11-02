
//! Excel file parsing framework

use std::{
    collections::HashMap,
    path::PathBuf,
    hash::Hash,
};

use calamine::{Reader, open_workbook, Xlsx, DataType};

// TODO: use serde for this.

/// A reader for a .xlsx file that parses a table
#[derive(Debug, Default)]
pub struct XlsxTableReader<H: Header> {
    header: HashMap<H, usize>
}

impl<H> XlsxTableReader<H>
    where
        H: Header + Eq + Hash
{

    /// create a new reader
    pub fn new() -> Self {
        Self {
            header: HashMap::new(),
            // TODO: hold not_matched columns for header parsing
        }
    }

    fn not_matched_header(&self) -> Option<Vec<String>> {
        let not_matched: Vec<String> = H::columns_to_match()
            .into_iter()
            .filter(|col| !self.header.contains_key(col))
            .map(|h| h.column_name())
            .collect();

        match not_matched.len() {
            0 => None,
            _ => Some(not_matched)
        }
    }

    fn is_header_matched(&self) -> bool {
        self.not_matched_header().is_none()
    }

    /// pares the header row of the table
    // TODO: support multiple rows
    pub fn parse_header(&mut self, row: &[DataType]) {
        for (i, col) in row.iter().enumerate() {
            if let Some(key) = H::match_header_column(col.get_string().unwrap()) {
                self.header.insert(key, i);
            }

            if self.is_header_matched() {
                break;
            }
        }
    }

    /// read an excel file, parsing the header and returning the parsed rows
    pub fn read_file(&mut self, path: PathBuf) -> anyhow::Result<Vec<anyhow::Result<H::Row>>> {
        let mut wb: Xlsx<_> = match open_workbook(path) {
            Ok(wb) => wb,
            Err(_) => return Err( anyhow!("failed ot open file") )
        };
        
        let rng = &wb.worksheets()[0].1;
        let mut rows = rng.rows().into_iter();

        self.parse_header(rows.next().unwrap());

        // validate header matched 
        if let Some(cols) = self.not_matched_header() {
            // TODO: specify which header columns not matched
            return Err( anyhow!("Not all header columns matched. Missing columns: `{}`", cols.join(", ")) );
        }

        let mut results = Vec::new();
        for row in rows {
            results.push(H::parse_row(&self.header, row));
        }

        return Ok(results);

        // Err(String::from("Failed to open first worksheet"))
    }
}

/// Header parser to aid the sheet parser
pub trait Header {
    /// type of Row that is returned by the parser during `read_file`
    type Row;

    /// returns the name of the column
    fn column_name(&self) -> String;
    /// tries to match a column, returns None if nothing matches
    fn match_header_column(column_text: &str) -> Option<Self> where Self: Sized;
    /// get a list of the columns to match in the header
    fn columns_to_match() -> Vec<Self> where Self: Sized;
    /// parse a data row with the parsed colum
    fn parse_row(header: &HashMap<Self, usize>, row: &[DataType]) -> anyhow::Result<Self::Row> where Self: Sized;
}