use binrw::{BinRead, BinWrite};

/// MAT-File Data Types, see Table 1-8, <https://www.mathworks.com/help/pdf_doc/matlab/matfile_format.pdf>
/// Left out exotic cases 2, 3, 4
#[derive(Clone, Copy, Debug, Eq, PartialEq, BinWrite)]
#[brw(repr = u8)]
pub enum Endian {
    /// The most significant byte is stored first.
    Big = 1,
    /// The least significant byte is stored first.
    Little = 0,
}

/// MAT-File Data Types, see Table 1-8, <https://www.mathworks.com/help/pdf_doc/matlab/matfile_format.pdf>
#[derive(PartialEq, Debug, BinRead, BinWrite, Clone, Copy)]
#[brw(repr = u8)]
pub enum MatFileDataTypes {
    MiDOUBLE = 0,
    MiSINGLE = 1,
    MiINT32 = 2,
    MiINT16 = 3,
    MiUINT16 = 4,
    MiUINT8 = 5,
}

/// MAT-File Data Types, see Table 1-8, <https://www.mathworks.com/help/pdf_doc/matlab/matfile_format.pdf>
#[derive(PartialEq, Debug, BinRead, BinWrite, Clone, Copy)]
#[brw(repr = u8)]
pub enum MatFileMatrixTypes {
    Full = 0,
    Text = 1,
    Sparse = 2,
}
