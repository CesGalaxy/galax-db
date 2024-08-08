use crate::data::DataType;

pub struct TableColumn {
    pub name: String,
    pub data_type: DataType
}

impl TableColumn {
    pub fn new(name: String, data_type: DataType) -> TableColumn {
        TableColumn {
            name,
            data_type
        }
    }
}
