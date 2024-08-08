use column::TableColumn;

use crate::data::DataValue;

pub mod column;

pub struct Table<const COLS: usize> {
    pub name: String,
    pub columns: [TableColumn; COLS],
    pub data: &'static [u8]
}

impl<const COLS: usize> Table<COLS> {
    pub fn new(name: String, columns: [TableColumn; COLS], data: &'static [u8]) -> Self {
        Table {
            name,
            columns,
            data
        }
    }

    pub fn get_row_size(&self) -> usize {
        // Summ the sizes of all columns types
        self.columns.iter().map(|column| column.data_type.get_size()).sum()
    }

    pub fn get_raw_row_at_index(&self, index: usize) -> &[u8] {
        let row_size = self.get_row_size();
        let start = index * row_size;
        let end = start + row_size;
        &self.data[start..end]
    }

    pub fn get_row_at_index(&self, index: usize) -> [DataValue; COLS] {
        let row = self.get_raw_row_at_index(index);

        self.columns.iter().enumerate().map(|(i, column)| {
            let start = i * column.data_type.get_size();
            let end = start + column.data_type.get_size();
            let value = &row[start..end];

            DataValue::try_from((&column.data_type, value)).expect("falied to get value from typed bytes")
        }).collect::<Vec<_>>().try_into().unwrap()
    }
}
