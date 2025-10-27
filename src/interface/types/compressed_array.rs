use crate::interface::variable::MatVariable;
use crate::parser::v7::types::compressed_array::CompressedArray7;

#[derive(Debug, Clone)]
pub struct CompressedArray {
    pub value: Box<MatVariable>,
}

impl From<CompressedArray7> for CompressedArray {
    fn from(value: CompressedArray7) -> Self {
        Self {
            value: Box::new(value.value().into()),
        }
    }
}
