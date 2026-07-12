use binrw::*;
use indexmap::IndexMap;

use crate::{MatFile, parser::v4::variable4::MatVariable4};

#[parser(reader, endian)]
pub fn parse_variable4() -> BinResult<IndexMap<String, MatVariable4>> {
    let mut map = IndexMap::new();

    loop {
        let data = match MatVariable4::read_options(reader, endian, ()) {
            Ok(d) => d,
            Err(err) if err.is_eof() => break,
            Err(err) => return Err(err),
        };

        let name = data.name();
        map.insert(name, data);
    }

    Ok(map)
}

#[binrw::writer(writer, endian)]
pub fn write_variable4(data: &IndexMap<String, MatVariable4>) -> BinResult<()> {
    for (_, val) in data.iter() {
        let _ = val.write_options(writer, endian, ());
    }

    Ok(())
}

#[binrw]
#[derive(Debug)]
pub struct MatFile4 {
    #[br(parse_with = parse_variable4)]
    #[bw(write_with = write_variable4)]
    pub data: IndexMap<String, MatVariable4>,
}

impl MatFile4 {
    pub fn new() -> Self {
        Self {
            data: IndexMap::new(),
        }
    }
}

impl Default for MatFile4 {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MatFile> for MatFile4 {
    fn from(value: MatFile) -> Self {
        let mut matfile = MatFile4::new();

        for (key, val) in value.into_iter() {
            let mut val4: MatVariable4 = val.into();
            val4.set_name(&key);

            matfile.data.insert(key, val4);
        }

        matfile
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
    const MATFILE4: [u8; 60] = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x40, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x62,
        0x00, 0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40,
    ];

    #[test]
    fn parse_matfile4() {
        let data = Cursor::new(&MATFILE4).read_le::<MatFile4>().unwrap();
        println!("Deserialized data: {:#?}", data);
    }
}
