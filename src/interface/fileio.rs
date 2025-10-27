use binrw::BinReaderExt;
use binrw::BinWrite;
use binrw::io::BufReader;
use binrw::io::Cursor;
use binrw::io::TakeSeekExt;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::interface::error::MatrwError;
use crate::interface::matfile::MatFile;
use crate::interface::variable::MatVariable;
use crate::parser::header;
use crate::parser::header::{MatFileHeader, MatFileVerFlag};
use crate::parser::v7::matfile7::MatFile7;

use super::types::compressed_array::CompressedArray;

/// Load MAT-file data from file.
///
/// Loads a MAT-file data from file using a provided path. In case of failure, the function returns
/// - [`MatrwError::IoError`], if the file cannot be found or read,
/// - [`MatrwError::BinrwError`], if the content of the file cannot be parsed,
/// - [`MatrwError::MatFile73Error`], if attempted to read a version 7.3 MAT-file, which is currently not supported.
///
/// Example
/// ```
/// use matrw::{load_matfile, MatFile};
///
/// let path = concat!(
///         env!("CARGO_MANIFEST_DIR"),
///         "/tests/example_v7.mat"
///         );
/// let matfile: MatFile = load_matfile(path)
///         .expect("Could not load MAT-file.");
/// ```
pub fn load_matfile(path: &str) -> Result<MatFile, MatrwError> {
    let f = File::open(path)?;
    let f_bytes = f.metadata().expect("Cannot read file metadata").len();
    let mut reader = BufReader::new(f);

    // Read the header to find out the file version and the endian
    let matheader = match reader.read_le::<MatFileHeader>() {
        Ok(header) => header,
        Err(err) => return Err(MatrwError::BinrwError(err)),
    };

    let endian = matheader.matfile_endian;
    let subsystem_offset = matheader.header_subsystem_data_offset_field;
    // Get the size to read out. In case the MAT-file contains objects, we want to ignore the
    // subsystem for now.
    let limit = if subsystem_offset != 0 {
        subsystem_offset
    } else {
        f_bytes
    } - header::HEADER_SIZE as u64;

    match matheader.matfile_ver {
        MatFileVerFlag::V7 => Ok(reader.take_seek(limit).read_type::<MatFile7>(endian)?.into()),
        MatFileVerFlag::V73 => Err(MatrwError::MatFile73Error),
    }
}

/// Write MAT-file
///
/// Example
/// ```
/// use matrw::{MatFile, matvar, save_matfile_v7};
///
/// // Create a new MatFile
/// let mut matfile = MatFile::new();
///
/// // Write MAT-file
/// save_matfile_v7("test.mat", matfile, false)
///         .expect("Could not write MAT-file");
///
/// # let _ = std::fs::remove_file("test.mat");
/// ```
pub fn save_matfile_v7(path: &str, matfile: MatFile, compress: bool) -> Result<(), MatrwError> {
    let f = File::create(path)?;
    let mut writer = BufWriter::new(f);

    let matheader = MatFileHeader::new(MatFileVerFlag::V7);

    let mut matfile = matfile;
    if compress {
        for (_, val) in matfile.iter_mut() {
            *val = MatVariable::Compressed(CompressedArray {
                value: Box::new(val.to_owned()),
            });
        }
    }

    let _ = matheader.write_options(&mut writer, matheader.matfile_endian, ());
    let _ = MatFile7::from(matfile).write_options(&mut writer, matheader.matfile_endian, ());
    let _ = writer.flush();

    Ok(())
}

/// Load MAT-file from u8
///
pub fn load_matfile_from_u8(data: &[u8]) -> Result<MatFile, MatrwError> {
    let mut cursor = Cursor::new(data);

    // Read the header to find out the file version and the endian
    let matheader = match cursor.read_le::<MatFileHeader>() {
        Ok(header) => header,
        Err(err) => return Err(MatrwError::BinrwError(err)),
    };

    let endian = matheader.matfile_endian;

    match matheader.matfile_ver {
        MatFileVerFlag::V7 => Ok(cursor.read_type::<MatFile7>(endian)?.into()),
        MatFileVerFlag::V73 => Err(MatrwError::MatFile73Error),
    }
}
