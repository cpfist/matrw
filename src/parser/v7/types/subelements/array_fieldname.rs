//! Module containing types for matching *Array Fieldname Subelements*.

use binrw::*;

use std::fmt::Debug;

use crate::parser::v7::flags::MatFileDataTypes;

#[parser(reader)]
fn parse_fieldnames(field_number: u32, length: u32) -> BinResult<Vec<Vec<u8>>> {
    let mut v = Vec::new();

    for _ in 0..field_number {
        let mut buf = vec![0; length as usize];
        reader.read_exact(&mut buf).unwrap();

        v.push(buf);
    }
    Ok(v)
}

#[derive(Clone)]
#[binrw]
pub struct ArrayFieldNames {
    /// Data type tag
    #[brw(pad_size_to = 2)]
    #[br(assert(data_type_length == MatFileDataTypes::MiINT32))]
    data_type_length: MatFileDataTypes,
    /// Number of bytes tag
    #[br(assert(data_size_length == 4))]
    data_size_length: u16,
    /// Data
    #[br(assert(length <= 64))] // Max length: 32 (Matlab) / 64 (Octave)
    length: u32,
    /// Data type tag
    #[brw(pad_size_to = 4)]
    #[br(assert(data_type == MatFileDataTypes::MiINT8))]
    data_type: MatFileDataTypes,
    /// Number of bytes tag
    data_size: u32,
    #[br(calc = data_size / length)]
    #[bw(ignore)]
    pub field_number: u32,
    /// Data
    #[br(parse_with = parse_fieldnames, args(field_number, length))]
    #[bw(pad_size_to = 8, if(*field_number > 0))]
    field_names: Vec<Vec<u8>>,
}

impl ArrayFieldNames {
    pub fn new(field_names: Vec<String>) -> Self {
        if !field_names.is_empty() {
            // Determine longest field name
            let mut max_length = field_names.iter().map(|x| x.len()).max().unwrap() + 1;

            // Field names can only have 63 characters. 64th is left
            // for null-termination byte
            if max_length > 63 {
                max_length = 64;
            }

            // If struct contains one field only, MATLAB sets
            // minimum length to 5
            if field_names.len() == 1 && max_length < 5 {
                max_length = 5;
            }

            // If struct contains two field only, MATLAB sets
            // minimum length to 3
            if field_names.len() == 2 && max_length < 3 {
                max_length = 3;
            }

            let mut field_names_conv = vec![];
            for field_name in field_names.iter() {
                let mut name_str = field_name.clone();

                // Filter out all non-ascii characters
                name_str = name_str.chars().filter(|c| c.is_ascii()).collect::<String>();

                // Truncate string to at most 63 characters
                // (leave one for the null-termination byte)
                name_str.truncate(63);

                let v_small = name_str.into_bytes();
                let mut v_large: Vec<u8> = vec![0; max_length];
                v_large.splice(0..v_small.len(), v_small);
                field_names_conv.push(v_large);
            }

            Self {
                data_type_length: MatFileDataTypes::MiINT32,
                data_size_length: 4u16,
                length: max_length as u32,
                data_type: MatFileDataTypes::MiINT8,
                data_size: (max_length * field_names.len()) as u32,
                field_number: field_names.len() as u32,
                field_names: field_names_conv,
            }
        } else {
            Self {
                data_type_length: MatFileDataTypes::MiINT32,
                data_size_length: 4u16,
                length: 1u32,
                data_type: MatFileDataTypes::MiINT8,
                data_size: 0u32,
                field_number: 0u32,
                field_names: vec![],
            }
        }
    }
    pub fn size(&self) -> u32 {
        let padding = if (self.data_size % 8) == 0 {
            0
        } else {
            8 - self.data_size % 8
        };
        16 + self.data_size + padding
    }
    pub fn fieldnames(&self) -> Vec<String> {
        let mut v = Vec::new();

        for buf in &self.field_names {
            let name = String::from_utf8(buf.clone())
                .unwrap()
                .trim_matches(char::from(0))
                .to_string();
            v.push(name);
        }

        v
    }
}

impl Debug for ArrayFieldNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbs = f.debug_struct("ArrayNameSmall");
        dbs.field("data_type_length", &self.data_type_length);
        dbs.field("data_size_length", &self.data_size_length);
        dbs.field("length", &self.length);
        dbs.field("data_type", &self.data_type);
        dbs.field("data_size", &self.data_size);
        dbs.field("field_number", &self.field_number);
        let mut v = Vec::new();
        for fname in self.field_names.iter() {
            let s = String::from_utf8(fname.clone()).unwrap();
            v.push(s);
        }
        dbs.field("field_names", &v);
        dbs.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinReaderExt;
    use binrw::io::Cursor; // A no_std reimplementation of std::io // extension traits for use with readers and writers

    #[test]
    fn fieldnames_no_fields() {
        // Deserialize
        let mut bin = Cursor::new(b"\x05\x00\x04\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayFieldNames>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.field_number == 0);

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayFieldNames::new(vec![]);
        data_new.write_le(&mut bin_new).unwrap();
        println!("  Original data: {:?}", bin);
        println!("Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn fieldnames_one_field_size1() {
        // Deserialize
        let mut bin = Cursor::new(b"\x05\x00\x04\x00\x05\x00\x00\x00\x01\x00\x00\x00\x05\x00\x00\x00\x61\x00\x00\x00\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayFieldNames>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.field_number == 1);

        let field_names = vec![
            String::from_utf8(data.field_names[0].clone())
                .unwrap()
                .trim_matches(char::from(0))
                .to_string(),
        ];
        assert!(field_names[0] == "a");

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayFieldNames::new(field_names);
        data_new.write_le(&mut bin_new).unwrap();
        println!("  Original data: {:?}", bin);
        println!("Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn fieldnames_two_fields_size1() {
        // Deserialize
        let mut bin = Cursor::new(b"\x05\x00\x04\x00\x03\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x61\x00\x00\x62\x00\x00\x00\x00");
        let data = bin.read_le::<ArrayFieldNames>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.field_number == 2);

        let field_names = vec![
            String::from_utf8(data.field_names[0].clone())
                .unwrap()
                .trim_matches(char::from(0))
                .to_string(),
            String::from_utf8(data.field_names[1].clone())
                .unwrap()
                .trim_matches(char::from(0))
                .to_string(),
        ];
        assert!(field_names[0] == "a");
        assert!(field_names[1] == "b");

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayFieldNames::new(field_names);
        data_new.write_le(&mut bin_new).unwrap();
        println!("  Original data: {:?}", bin);
        println!("Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn fieldnames_two_fields_size23() {
        // Deserialize
        let mut bin = Cursor::new(b"\x05\x00\x04\x00\x18\x00\x00\x00\x01\x00\x00\x00\x30\x00\x00\x00\x6c\x6f\x6e\x67\x5f\x66\x69\x65\x6c\x64\x5f\x6e\x61\x6d\x65\x00\x00\x00\x00\x00\x00\x00\x00\x00\x6c\x6f\x6f\x6f\x6f\x6f\x6f\x6f\x6e\x67\x65\x72\x5f\x66\x69\x65\x6c\x64\x5f\x6e\x61\x6d\x65\x00");
        let data = bin.read_le::<ArrayFieldNames>().unwrap();
        println!("Deserialized data: {:#?}", &data);
        assert!(data.field_number == 2);

        let field_names = vec![
            String::from_utf8(data.field_names[0].clone())
                .unwrap()
                .trim_matches(char::from(0))
                .to_string(),
            String::from_utf8(data.field_names[1].clone())
                .unwrap()
                .trim_matches(char::from(0))
                .to_string(),
        ];
        assert!(field_names[0] == "long_field_name");
        assert!(field_names[1] == "looooooonger_field_name");

        // Serialize
        let mut bin_new = Cursor::new(vec![]);
        let data_new = ArrayFieldNames::new(field_names);
        data_new.write_le(&mut bin_new).unwrap();
        println!("  Original data: {:?}", bin);
        println!("Serialized data: {:?}", bin_new);
        assert!(bin_new.into_inner().to_vec() == bin.into_inner().to_vec());
    }

    #[test]
    fn fieldnames_truncate() {
        let test_vec = vec![
            ['a'; 100].iter().collect(),
            ['b'; 120].iter().collect(),
            ['c'; 42].iter().collect(),
        ];

        let data_new = ArrayFieldNames::new(test_vec);
        println!("{:#?}", data_new);

        assert!(data_new.length == 64);
        assert!(data_new.data_size == 192);
        assert!(data_new.field_number == 3);
    }
}
