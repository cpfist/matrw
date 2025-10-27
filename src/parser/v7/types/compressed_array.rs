use crate::interface::types::compressed_array::CompressedArray;
use crate::parser::v7::flags::MatFileDataTypes;
use crate::parser::v7::variable7::MatVariable7;
use std::io::{Cursor, Read, Seek, Write};
use std::ops::Deref;

use binrw::io::TakeSeekExt;
use binrw::*;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

fn uncompress_data<S: Read + Seek>(data: S) -> Cursor<Vec<u8>> {
    let mut zlib = ZlibDecoder::new(data);

    let mut buf = vec![];
    zlib.read_to_end(&mut buf).unwrap();

    Cursor::new(buf)
}

#[binrw::writer(writer, endian)]
#[allow(clippy::borrowed_box)]
fn compress_data(value: &Box<MatVariable7>) -> BinResult<()> {
    // Initialize encoder
    let mut zlib = ZlibEncoder::new(Vec::new(), Compression::new(9));

    // Compress value
    let mut c1 = Cursor::new(vec![]);
    let _ = value.write_options(&mut c1, endian, ());
    let _ = zlib.write_all(&c1.into_inner());
    let compressed = zlib.finish().unwrap();

    // Calculate size
    let size = compressed.len() as u32;
    let mut c2 = Cursor::new(vec![]);
    let _ = size.write_options(&mut c2, endian, ());

    // Write out
    let _ = writer.write_all(&c2.into_inner());
    let _ = writer.write_all(&compressed);

    Ok(())
}

#[binrw]
#[derive(Debug, Clone)]
pub struct CompressedArray7 {
    #[br(assert(data_type == MatFileDataTypes::MiCOMPRESSED))]
    #[brw(pad_size_to = 4)]
    data_type: MatFileDataTypes,
    #[bw(ignore)]
    num_bytes: u32,
    #[br(map_stream = |inner| uncompress_data(inner.take_seek(num_bytes as u64)))]
    #[bw(write_with = compress_data)]
    value: Box<MatVariable7>,
}

impl CompressedArray7 {
    pub fn new(value: MatVariable7) -> Self {
        Self {
            data_type: MatFileDataTypes::MiCOMPRESSED,
            num_bytes: 0u32,
            value: Box::new(value),
        }
    }
    pub fn set_name(&mut self, name: &str) {
        self.value.set_name(name);
    }
    pub fn name(&self) -> String {
        self.value.name()
    }
    pub fn value(self) -> MatVariable7 {
        *self.value
    }
}

impl From<CompressedArray> for CompressedArray7 {
    fn from(value: CompressedArray) -> Self {
        Self::new(value.value.deref().clone().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::v7::types::numeric_array::{NumericArray7, NumericArrayNew};
    use binrw::BinReaderExt;
    use binrw::io::Cursor; // A no_std reimplementation of std::io // extension traits for use with readers and writers

    /// Binary representation of a mat file containing a variable `var_double` with double values.
    /// To reproduce, in a MATLAB session with a clean workspace run
    /// `>> a = [11,22,33,44,55,66,77,88,99];`
    /// `>> save('example.mat', 'a');`
    ///
    const DATA_F64_1: [u8; 55] = [
        0x0f, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0x78, 0x9c, 0xe3, 0x63, 0x60, 0x60, 0x70, 0x00, 0x62,
        0x36, 0x20, 0xe6, 0x80, 0xd2, 0x20, 0xc0, 0x0a, 0xe5, 0x33, 0x02, 0x31, 0x27, 0x98, 0x66, 0x64, 0x48,
        0x04, 0xd2, 0x4c, 0x50, 0x3e, 0xb7, 0x98, 0xa2, 0x8e, 0xb9, 0x93, 0x6f, 0x44, 0x32, 0x54, 0x3d, 0x00,
        0x3e, 0x02, 0x02, 0xd7,
    ];
    const VAR_F64_1: &[f64; 9] = &[11., 22., 33., 44., 55., 66., 77., 88., 99.];

    #[test]
    fn deserialize_double_compressed() {
        let mut bin = Cursor::new(&DATA_F64_1);
        let data = bin.read_le::<CompressedArray7>().unwrap();
        println!("Deserialized data: {:#?}", data);
        // let val: &Vec<f64> = data.value.as_vec_f64().unwrap();
        // assert!(val == VAR_F64_1);
    }

    #[test]
    fn serialize_double_compressed() {
        let mut bin = Cursor::new(vec![]);
        let mut value = MatVariable7::Numeric(NumericArray7::new(vec![1, 9], VAR_F64_1.to_vec(), None));
        value.set_name("a");
        let data = CompressedArray7::new(value);
        println!("data: {:#?}", &data);
        data.write_le(&mut bin).unwrap();

        let inner = bin.into_inner();
        println!("Orig bin: {:?}", DATA_F64_1);
        println!("Ser  bin: {:?}", &inner);
        // assert!(inner == DATA_F64_1);
    }
}
