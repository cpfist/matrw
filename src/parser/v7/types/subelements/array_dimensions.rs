//! Module containing types for matching *Array Dimension Subelements*.

use binrw::*;

use crate::parser::v7::flags::MatFileDataTypes;

#[binrw]
#[derive(Debug, Clone)]
pub enum ArrayDimensions {
    DataNormal(ArrayDimensionsNormal),
    DataSmall(ArrayDimensionsSmall),
}

impl ArrayDimensions {
    pub fn new(dim: Vec<u32>) -> Self {
        if dim.len() <= 1 {
            Self::DataSmall(ArrayDimensionsSmall::new(dim))
        } else {
            Self::DataNormal(ArrayDimensionsNormal::new(dim))
        }
    }
    pub fn size(&self) -> u32 {
        match self {
            ArrayDimensions::DataNormal(v) => v.size(),
            ArrayDimensions::DataSmall(v) => v.size() as u32,
        }
    }
    pub fn dim(&self) -> &Vec<u32> {
        match self {
            ArrayDimensions::DataNormal(v) => &v.dimensions,
            ArrayDimensions::DataSmall(v) => &v.dimensions,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            ArrayDimensions::DataNormal(v) => v.dimensions.clone().into_iter().product::<u32>() == 0,
            ArrayDimensions::DataSmall(v) => v.dimensions.clone().into_iter().product::<u32>() == 0,
        }
    }
}

#[derive(Debug, Clone)]
#[binrw]
pub struct ArrayDimensionsNormal {
    /// Data type tag
    #[brw(pad_size_to = 2)]
    #[br(assert(data_type == MatFileDataTypes::MiINT32))]
    data_type: MatFileDataTypes,
    #[br(assert(check == 0))]
    check: u16,
    /// Number of bytes tag
    data_size: u32,
    /// Array dimensions
    #[br(count = (data_size / 4) as usize)]
    dimensions: Vec<u32>,
}

impl ArrayDimensionsNormal {
    pub fn new(dim: Vec<u32>) -> Self {
        Self {
            data_type: MatFileDataTypes::MiINT32,
            check: 0_u16,
            data_size: (4 * dim.len()) as u32,
            dimensions: dim,
        }
    }
    pub fn size(&self) -> u32 {
        8 + self.data_size + self.data_size % 8
    }
}

#[derive(Debug, Clone)]
#[binrw]
pub struct ArrayDimensionsSmall {
    /// Data type tag
    #[brw(pad_size_to = 2)]
    #[br(assert(data_type == MatFileDataTypes::MiINT32))]
    data_type: MatFileDataTypes,
    /// Number of bytes tag
    data_size: u16,
    /// Array dimensions
    #[br(count = (data_size / 4) as usize)]
    dimensions: Vec<u32>,
}

impl ArrayDimensionsSmall {
    pub fn new(dim: Vec<u32>) -> Self {
        Self {
            data_type: MatFileDataTypes::MiINT32,
            data_size: (4 * dim.len()) as u16,
            dimensions: dim,
        }
    }
    pub fn size(&self) -> u16 {
        4 + self.data_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinReaderExt;
    use binrw::io::Cursor; // A no_std reimplementation of std::io // extension traits for use with readers and writers

    #[test]
    fn dimension_2_3() {
        // Deserialize
        let mut bin = Cursor::new(b"\x05\x00\x00\x00\x08\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00");
        let data = bin.read_le::<ArrayDimensions>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.dim() == &vec![2, 3]);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayDimensions::new(data.dim().clone());
        data_new.write_le(&mut bin_new).unwrap();
        println!("Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn dimension_0_0() {
        // Deserialize
        let mut bin = Cursor::new(b"\x05\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayDimensions>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.dim() == &vec![0, 0]);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayDimensions::new(data.dim().clone());
        data_new.write_le(&mut bin_new).unwrap();
        println!("Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn dimension_2_3_2() {
        // Deserialize
        let mut bin =
            Cursor::new(b"\x05\x00\x00\x00\x0c\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00\x02\x00\x00\x00");
        let data = bin.read_le::<ArrayDimensions>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(*data.dim() == vec![2, 3, 2]);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayDimensions::new(data.dim().clone());
        data_new.write_le(&mut bin_new).unwrap();
        println!("Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }
}
