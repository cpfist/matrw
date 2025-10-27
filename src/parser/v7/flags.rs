use binrw::*;

/// MAT-File Data Types, see Table 1-1, <https://www.mathworks.com/help/pdf_doc/matlab/matfile_format.pdf>
#[derive(PartialEq, Debug, BinRead, BinWrite, Clone)]
#[brw(repr = u8)]
pub enum MatFileDataTypes {
    MiINT8 = 1,
    MiUINT8 = 2,
    MiINT16 = 3,
    MiUINT16 = 4,
    MiINT32 = 5,
    MiUINT32 = 6,
    MiSINGLE = 7,
    MiDOUBLE = 9,
    MiINT64 = 12,
    MiUINT64 = 13,
    MiMATRIX = 14,
    MiCOMPRESSED = 15,
    MiUTF8 = 16,
    MiUTF16 = 17,
    MiUTF32 = 18,
}

/// MATLAB Array Types (Classes), see Table 1-3, <https://www.mathworks.com/help/pdf_doc/matlab/matfile_format.pdf>
#[derive(PartialEq, Debug, BinRead, BinWrite, Clone, Copy, Default)]
#[brw(repr = u8)]
pub enum MatlabArrayTypes {
    MxCELLCLASS = 1,
    MxSTRUCTCLASS = 2,
    MxOBJECTCLASS = 3,
    MxCHARCLASS = 4,
    MxSPARSECLASS = 5,
    #[default]
    MxDOUBLECLASS = 6,
    MxSINGLECLASS = 7,
    MxINT8CLASS = 8,
    MxUINT8CLASS = 9,
    MxINT16CLASS = 10,
    MxUINT16CLASS = 11,
    MxINT32CLASS = 12,
    MxUINT32CLASS = 13,
    MxINT64CLASS = 14,
    MxUINT64CLASS = 15,
    MxHANDLECLASS = 16,
    MxOPAQUECLASS = 17,
}
