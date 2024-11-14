use crate::data::DataType;
use crate::store::relationship::Relationship;
use crate::store::table::{Table, TableMetadata};

mod store;
mod data;

fn main() {
    let data = [21, 0, 22, 1];
    
    let table = Table {
        data: &data,
        relationships: [
            Relationship {
                name: String::from("age"),
                data_type: DataType::Int8,
            },
            Relationship {
                name: String::from("vip"),
                data_type: DataType::Bool,
            },
        ],
        metadata: TableMetadata { name: String::from("my_table") }
    };
    
    println!("Hello, world!");
    
    println!("Table name: {}", table.metadata.name);

    println!("At 0: {:?}", table.get_at_index(0));
    println!("At 1: {:?}", table.get_at_index(1));
}
