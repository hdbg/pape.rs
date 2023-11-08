use crate::specification::*;
use chrono::offset::Utc;
use num_traits::int::PrimInt;
use std::collections::HashMap;

mod collections;
pub use collections::enumset::EnumSet;

pub struct Version<T: PrimInt> {
    pub major: T,
    pub minor: T,
}

pub struct Section {
    pub name: String,
    pub virtual_size: u32,
    pub characteristics: EnumSet<SectionFlags, u32>,
    pub data: Option<Vec<u8>>,
}
pub struct HeaderSizeInfo {
    pub of_code: u32,
    pub of_initialized_data: u32,
    pub of_unintialized_data: u32,
}
pub struct HeaderMemInfo {
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
}

pub struct COFF {
    pub machine: MachineType,
    pub time_stamp: chrono::DateTime<Utc>,
    pub characteristics: EnumSet<Characteristics, u16>,
}

pub struct OptionalVersionsInfo {
    pub linker_ver: Version<u8>,
    pub os_version: Version<u16>,
    pub image_version: Version<u16>,
    pub subsystem_version: Version<u16>,
}

pub struct Optional {
    pub sizes: HeaderSizeInfo,
    pub entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: Option<u32>,

    pub versions: OptionalVersionsInfo,

    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,

    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub subsystem: Subsystem,
    pub checksum: u32,

    pub dll_characteristics: EnumSet<Characteristics, u16>,
    pub mem_info: HeaderMemInfo,
}
pub struct Image {
    pub magic: OptionalMagic,

    pub coff: COFF,
    pub optional: Option<Optional>,

    pub sections: Vec<Section>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            magic: OptionalMagic::PE32,
            coff: COFF {
                machine: MachineType::Unknown,
                time_stamp: chrono::DateTime::<Utc>::UNIX_EPOCH,
                characteristics: EnumSet::new(),
            },
            optional: None,
            sections: Vec::new(),
        }
    }
}
