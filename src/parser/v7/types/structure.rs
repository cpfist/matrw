use binrw::*;

use crate::interface::types::structure::Structure;
use crate::parser::v7::flags::{MatFileDataTypes, MatlabArrayTypes};
use crate::parser::v7::types::subelements::array_dimensions::ArrayDimensions;
use crate::parser::v7::types::subelements::array_fieldname::ArrayFieldNames;
use crate::parser::v7::types::subelements::array_flags::ArrayProps;
use crate::parser::v7::types::subelements::array_name::ArrayName;
use crate::parser::v7::variable7::MatVariable7;

use super::subelements::array_flags::ArrayFlagBits;

#[binrw]
#[derive(Debug, Clone)]
#[br(assert(data_type == MatFileDataTypes::MiMATRIX &&
            props.array_class == MatlabArrayTypes::MxSTRUCTCLASS))]
pub struct Structure7 {
    #[brw(pad_size_to = 4)]
    data_type: MatFileDataTypes,
    #[bw(calc = self.size_data())]
    _num_bytes: u32,
    props: ArrayProps,
    #[brw(align_after = 8)]
    #[br(assert(dimensions.dim().iter().product::<u32>() == 1))]
    dimensions: ArrayDimensions,
    #[brw(align_after = 8)]
    name: ArrayName,
    #[brw(align_after = 8)]
    fieldnames: ArrayFieldNames,
    #[br(count = fieldnames.fieldnames().len())]
    value: Vec<MatVariable7>,
}

impl Structure7 {
    pub fn new(field_strings: Vec<String>, value: Vec<MatVariable7>) -> Self {
        let name = "".to_string();

        let props = ArrayProps::new(
            MatlabArrayTypes::MxSTRUCTCLASS,
            ArrayFlagBits::new(false, false, false),
            0,
        );
        let dimensions = ArrayDimensions::new(vec![1, 1]);
        let name = ArrayName::new(name);
        let fieldnames = ArrayFieldNames::new(field_strings);

        Self {
            data_type: MatFileDataTypes::MiMATRIX,
            props,
            dimensions,
            name,
            fieldnames,
            value,
        }
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = ArrayName::new(name.to_string())
    }
    pub fn name(&self) -> String {
        self.name.name()
    }
    pub fn size_data(&self) -> u32 {
        let mut num_bytes = 0;
        num_bytes += self.props.size();
        num_bytes += self.dimensions.size();
        num_bytes += self.name.size();
        num_bytes += self.fieldnames.size();
        for val in self.value.iter() {
            num_bytes += val.size() as u32
        }

        num_bytes
    }
    pub fn size(&self) -> usize {
        self.size_data() as usize + 8
    }
    pub fn fieldnames(&self) -> Vec<String> {
        self.fieldnames.fieldnames()
    }
    pub fn value(self) -> Vec<MatVariable7> {
        self.value
    }
}

impl From<Structure> for Structure7 {
    fn from(value: Structure) -> Self {
        let mut fieldnames = Vec::new();
        let mut values = Vec::new();
        for (key, val) in value.value.into_iter() {
            fieldnames.push(key);
            values.push(val.into());
        }

        Self::new(fieldnames, values)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::parser::v7::types::numeric_array::{NumericArray7, NumericArrayNew};

    use super::*;
    use binrw::BinReaderExt;
    use binrw::io::Cursor; // A no_std reimplementation of std::io // extension traits for use with readers and writers

    /// Binary representation of a mat file containing a variable `S` with a structure.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> S = struct();`
    /// `>> save('example.mat', 'S', '-nocompression');`
    ///
    const DATA_STRUCTURE_EMPTY: [u8; 64] = [
        0x0e, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x53, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04,
        0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn deserialize_structure_empty() {
        let data = Cursor::new(&DATA_STRUCTURE_EMPTY)
            .read_le::<Structure7>()
            .unwrap();
        println!("{:#?}", &data);
    }

    #[test]
    fn serialize_structure_empty() {
        let mut bin = Cursor::new(vec![]);
        let values = vec![];

        let mut data = Structure7::new(vec![], values);
        data.set_name("S");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_STRUCTURE_EMPTY);
        println!("Ser  bin: {:?}", &inner);
        assert!(inner == DATA_STRUCTURE_EMPTY);
    }

    /// Binary representation of a mat file containing a variable `A` with a cell array.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> s.a = 42;`
    /// `>> s.b = pi;`
    /// `>> save('example.mat', 's', '-nocompression');`
    ///
    const DATA_STRUCTURE_1: [u8; 192] = [
        0x0e, 0x00, 0x00, 0x00, 0xb8, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x73, 0x00, 0x00, 0x00, 0x05, 0x00, 0x04,
        0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x61, 0x00, 0x00, 0x62,
        0x00, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08,
        0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x08, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x01, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x18, 0x2d, 0x44,
        0x54, 0xfb, 0x21, 0x09, 0x40,
    ];

    #[test]
    fn deserialize_structure1() {
        let data = Cursor::new(&DATA_STRUCTURE_1).read_le::<Structure7>().unwrap();
        println!("{:#?}", &data);

        // let field = data.value["a"].as_vec_f64().unwrap();
        // println!("{:#?}", field);
    }

    #[test]
    fn serialize_structure1() {
        let mut bin = Cursor::new(vec![]);
        let values = vec![
            MatVariable7::Numeric(NumericArray7::new(vec![1, 1], vec![42.], None)),
            MatVariable7::Numeric(NumericArray7::new(vec![1, 1], vec![PI], None)),
        ];

        let mut data = Structure7::new(vec!["a".to_string(), "b".to_string()], values);
        data.set_name("s");
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_STRUCTURE_1);
        println!("Ser  bin: {:?}", &inner);
        assert!(inner == DATA_STRUCTURE_1);
    }
}
