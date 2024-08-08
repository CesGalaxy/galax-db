use galax_db::{data::DataType, table::{column::TableColumn, Table}};

// 1 BYTE = 8 bits

fn main() {
    println!("Hello, world!");

    let data = &[
        1, 12,
        0, 11,
        1, 10,
        0, 9,
        1, 8,
        0, 7,
    ];

    let columns = [
        TableColumn::new("my_column".to_string(), DataType::Bool),
        TableColumn::new("my_column2".to_string(), DataType::U8)
    ];

    let table = Table::new("my_table".to_string(), columns, data);

    println!("Table name: {:?}", table.get_row_at_index(3));
}
