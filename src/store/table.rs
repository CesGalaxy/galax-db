use crate::data::DataValue;
use crate::store::relationship::Relationship;

pub struct Table<'a, const RELATIONSHIPS: usize> {
    pub data: &'a [u8],
    pub relationships: [Relationship; RELATIONSHIPS],
    pub metadata: TableMetadata,
}

impl<const RELATIONSHIPS: usize> Table<'_, RELATIONSHIPS> {
    pub fn row_size(&self) -> usize {
        let mut count = 0;
        
        // For each relationship, get the size
        for relationship in &self.relationships {
            count += relationship.data_type.size();
        }
        
        count
    }
    
    pub fn get_at_index(&self, index: usize) -> [DataValue; RELATIONSHIPS] {
        let row_size = self.row_size();
        
        let start = index * row_size;
        let end = start + row_size;
        
        let mut bytes = &self.data[start..end];
        
        let mut row = [DataValue::Null; RELATIONSHIPS];
        
        for (i, relationship) in self.relationships.iter().enumerate() {
            row[i] = relationship.data_type.read(&mut bytes)
        }
        
        row
    }
}

pub struct TableMetadata {
    pub(crate) name: String
}