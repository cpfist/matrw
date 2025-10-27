use binrw::*;

use crate::parser::v7::flags::{MatFileDataTypes, MatlabArrayTypes};
use crate::parser::v7::types::subelements::array_dimensions::ArrayDimensions;
use crate::parser::v7::types::subelements::array_flags::ArrayProps;
use crate::parser::v7::types::subelements::array_name::ArrayName;
use crate::parser::v7::variable7::MatVariable7;

#[binrw]
#[derive(Debug, Clone)]
#[br(assert(data_type == MatFileDataTypes::MiMATRIX &&
            props.array_class == MatlabArrayTypes::MxOPAQUECLASS))]
pub struct ObjectMCOS7 {
    #[brw(pad_size_to = 4)]
    data_type: MatFileDataTypes,
    num_bytes: u32,
    props: ArrayProps,
    #[brw(align_after = 8)]
    name: ArrayName,
    t1: u16,
    t2: u16,
    label: [u8; 4],
    //
    #[br(align_after = 8)]
    type_name: ArrayName,
    var: Box<MatVariable7>,
}

impl ObjectMCOS7 {
    pub fn set_name(&mut self, name: &str) {
        self.name = ArrayName::new(name.to_string())
    }
    pub fn name(&self) -> String {
        self.name.name()
    }
}

#[binrw]
#[derive(Debug, Clone)]
#[br(assert(data_type == MatFileDataTypes::MiMATRIX &&
            props.array_class == MatlabArrayTypes::MxHANDLECLASS))]
pub struct ObjectHandle7 {
    #[brw(pad_size_to = 4)]
    data_type: MatFileDataTypes,
    num_bytes: u32,
    props: ArrayProps,
    #[brw(align_after = 8)]
    dimensions: ArrayDimensions,
    #[brw(align_after = 8)]
    name: ArrayName,
    var: Box<MatVariable7>,
}

impl ObjectHandle7 {
    pub fn set_name(&mut self, name: &str) {
        self.name = ArrayName::new(name.to_string())
    }
    pub fn name(&self) -> String {
        self.name.name()
    }
}
