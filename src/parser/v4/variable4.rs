use binrw::*;

use crate::{MatVariable, parser::v4::types::numeric_array::NumericArray4};

#[binrw]
#[derive(Debug, Clone)]
pub struct MatVariable4(pub NumericArray4);

impl MatVariable4 {
    pub fn set_name(&mut self, name: &str) {
        self.0.set_name(name);
    }
    pub fn name(&self) -> String {
        self.0.name()
    }
}

impl From<MatVariable> for MatVariable4 {
    fn from(value: MatVariable) -> Self {
        match value {
            MatVariable::NumericArray(v) => MatVariable4(NumericArray4::from(v)),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinReaderExt;
    use binrw::io::Cursor; // A no_std reimplementation of std::io // extension traits for use with readers and writers

    /// Binary representation of a mat file containing two variables 'a' and 'b'.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = 42;`
    /// `>> b = pi;`
    /// `>> save('example.mat', '-v4', 'a', 'b');`
    ///
    const MATFILE4_F64: [u8; 60] = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x40, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x62,
        0x00, 0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40,
    ];

    #[test]
    fn parse_matvariable4() {
        let data = Cursor::new(&MATFILE4_F64).read_le::<[MatVariable4; 2]>().unwrap();
        println!("Deserialized data: {:#?}", data);
    }

    /// Binary representation of a mat file containing one variable 'a'.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = 'c';`
    /// `>> save('example.mat', '-v4', 'a');`
    ///
    const MATFILE4_CHAR: [u8; 30] = [
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x61, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xc0, 0x58, 0x40
    ];

    #[test]
    fn parse_matvariable4_char() {
        let data = Cursor::new(&MATFILE4_CHAR).read_le::<[MatVariable4; 1]>().unwrap();
        println!("Deserialized data: {:#?}", data);
    }
}
