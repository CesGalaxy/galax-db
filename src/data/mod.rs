use std::fmt::Debug;

pub enum DataType {
    Int8,
    Int16,
    Int32,
    Int64,
    Bool,
}

impl DataType {
    pub fn size(&self) -> usize {
        match self {
            DataType::Int8 => 1,
            DataType::Int16 => 2,
            DataType::Int32 => 4,
            DataType::Int64 => 8,
            DataType::Bool => 1,
        }
    }
    
    pub fn read(&self, source: &mut &[u8]) -> DataValue {
        match self {
            DataType::Int8 => {
                let (int_bytes, rest) = source.split_at(1);
                *source = rest;
                DataValue::Int8(i8::from_be_bytes(int_bytes.try_into().unwrap()))
            },
            DataType::Int16 => {
                let (int_bytes, rest) = source.split_at(2);
                *source = rest;
                DataValue::Int16(i16::from_be_bytes(int_bytes.try_into().unwrap()))
            },
            DataType::Int32 => {
                let (int_bytes, rest) = source.split_at(4);
                *source = rest;
                DataValue::Int32(i32::from_be_bytes(int_bytes.try_into().unwrap()))
            },
            DataType::Int64 => {
                let (int_bytes, rest) = source.split_at(8);
                *source = rest;
                DataValue::Int64(i64::from_be_bytes(int_bytes.try_into().unwrap()))
            },
            DataType::Bool => {
                let (bool_byte, rest) = source.split_at(1);
                *source = rest;
                DataValue::Bool(bool_byte[0] != 0)
            },
        }
    }
}

#[derive(Copy, Clone)]
pub enum DataValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Bool(bool),
    Null,
}

impl Debug for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataValue::Int8(value) => write!(f, "{}::i8", value),
            DataValue::Int16(value) => write!(f, "{}::i16", value),
            DataValue::Int32(value) => write!(f, "{}::i32", value),
            DataValue::Int64(value) => write!(f, "{}::i64", value),
            DataValue::Bool(value) => write!(f, "{}::bool", value),
            DataValue::Null => write!(f, "NULL"),
        }
    }
}