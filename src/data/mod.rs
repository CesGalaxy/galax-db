pub enum DataType {
    Bool,
    U8
}

impl DataType {
    /// The amount of bytes that takes to store a single value of this type.
    pub fn get_size(&self) -> usize {
        match self {
            Self::Bool => 1,
            Self::U8 => 1
        }
    }
}

impl TryFrom<u8> for DataType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Bool),
            1 => Ok(Self::U8),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum DataValue {
    Bool(bool),
    U8(u8)
}

impl TryFrom<(&DataType, &[u8])> for DataValue {
    type Error = ();

    fn try_from((data_type, bytes): (&DataType, &[u8])) -> Result<Self, ()> {
        match data_type {
            DataType::Bool => {
                if bytes.len() != 1 {
                    return Err(());
                }
                Ok(Self::Bool(bytes[0] != 0))
            },
            DataType::U8 => {
                if bytes.len() != 1 {
                    return Err(());
                }
                Ok(Self::U8(bytes[0]))
            }
        }
    }
}
