use strum::EnumIter;

#[repr(C, packed)]
pub struct DosHeader {
    pub e_magic: u16,
    pub _padding: [u8; 58],
    pub e_lfanew: u32,
}

pub const DOS_HEADER_MAGIC: u16 = 0x5a4d;
pub const PE_HEADER_MAGIC: u32 = 0x50450000;

#[repr(u16)]
#[derive(EnumIter, Clone, Copy)]
pub enum MachineType {
    Unknown = 0,
    Alpha = 0x184,
    Alpha64_or_AXP64 = 0x284,
    AM33 = 0x1d3,
    AMD64 = 0x8664,
    ARM = 0x1c0,
    ARM64 = 0xaa64,
    ARMNT = 0x1c4,
    EBC = 0xebc,
    i386 = 0x14c,
    IA64 = 0x200,
    LoongArch32 = 0x6232,
    LoongArch64 = 0x6424,
    M32R = 0x9041,
    MIPS16 = 0x266,
    MIPSFPU = 0x366,
    MIPSFPU16 = 0x466,
    PowerPC = 0x1f0,
    PowerPCFP = 0x1f1,
    R4000 = 0x166,
    RISCV32 = 0x5032,
    RISCV64 = 0x5064,
    RISCV128 = 0x5128,
    SH3 = 0x1a2,
    SH3DSP = 0x1a3,
    SH4 = 0x1a6,
    SH5 = 0x1a8,
    Thumb = 0x1c2,
    WCEMIPSV2 = 0x169,
}

#[repr(u16)]
#[derive(EnumIter, Clone, Copy)]
pub enum Characteristics {
    RelocsStripped = 0x0001,
    ExecutableImage = 0x0002,
    LineNumsStripped = 0x0004,
    LocalSymsStripped = 0x0008,
    AgressiveWSTrim = 0x0010,
    LargeAddressAware = 0x0020,
    LittleEndian = 0x0080,
    Machine32bit = 0x0100,
    DebugStripped = 0x0200,
    RemovableRunFromSwap = 0x0800,
    FileSystem = 0x1000,
    DLL = 0x2000,
    UpSystemOnly = 0x4000,
    BigEndian = 0x8000,
}
impl Into<u16> for Characteristics {
    fn into(self) -> u16 {
        self as u16
    }
}

#[repr(C, packed)]
pub struct CoffHeader {
    pub magic: u32,
    pub machine: MachineType,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum OptionalMagic {
    PE32 = 0x10b,
    PE32Plus = 0x20b,
    ROM = 0x107,
}

#[repr(C, packed)]
pub struct OptionalHeadersStandard {
    pub magic: OptionalMagic,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32, // should not read if pe32+
}

#[repr(u16)]
#[derive(EnumIter, Copy, Clone)]
pub enum Subsystem {
    Unknown = 0,
    Native = 1,
    WindowsGUI = 2,
    WindowsCUI = 3,
    OS2CUI = 5,
    POSIXCUI = 7,
    NativeWindows = 8,
    WindowsCE = 9,
    EFIApplication = 10,
    EFIBootServiceDriver = 11,
    EFIRuntimeDriver = 12,
    EFIROM = 13,
    XBOX = 14,
    WindowsBootApplication = 16,
}

#[repr(u16)]
#[derive(EnumIter, Copy, Clone)]
pub enum DllCharacteristics {
    HighEntropyVA = 0x0020,
    DynamicBase = 0x0040,
    ForceIntegrity = 0x0080,
    NXCompatible = 0x0100,
    NoIsolation = 0x0200,
    NoSEH = 0x0400,
    NoBind = 0x0800,
    AppContainer = 0x1000,
    WDMDriver = 0x2000,
    GuardCF = 0x4000,
    TerminalServerAware = 0x8000,
}

pub trait WindowsHeaderNum {}
impl WindowsHeaderNum for u32 {}
impl WindowsHeaderNum for u64 {}

#[derive(Clone)]
pub enum DataDirectories {
    ExportTable,
    ImportTable,
    ResourceTable,
    ExceptionTable,
    CertificateTable,
    BaseRelocatingTable,
    Debug,
    _Architecture,
    GlobalPtr,
    TLSTable,
    LoadConfigTable,
    BoundImport,
    IAT,
    DelayImportDescriptor,
    CLRRuntimeHeader,
    _Reserved,
}

impl Into<usize> for DataDirectories {
    fn into(self) -> usize {
        match self {
            DataDirectories::ExportTable => 0,
            DataDirectories::ImportTable => 1,
            DataDirectories::ResourceTable => 2,
            DataDirectories::ExceptionTable => 3,
            DataDirectories::CertificateTable => 4,
            DataDirectories::BaseRelocatingTable => 5,
            DataDirectories::Debug => 6,
            DataDirectories::_Architecture => 7,
            DataDirectories::GlobalPtr => 8,
            DataDirectories::TLSTable => 9,
            DataDirectories::LoadConfigTable => 10,
            DataDirectories::BoundImport => 11,
            DataDirectories::IAT => 12,
            DataDirectories::DelayImportDescriptor => 13,
            DataDirectories::CLRRuntimeHeader => 14,
            DataDirectories::_Reserved => 15,
        }
    }
}
impl std::hash::Hash for DataDirectories {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let val: usize = self.clone().into();
        val.hash(state);
    }
}

#[repr(C, packed)]
pub struct DataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}
const DATA_DIRS_COUNT: usize = 16;

#[repr(C, packed)]
pub struct OptionalHeadersWindows<T: WindowsHeaderNum> {
    pub image_base: T,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_versioin: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub _win32_version_value: u32, // must be zero
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: Subsystem,
    pub dll_characteristics: DllCharacteristics,
    pub size_of_stack_reserve: T,
    pub size_of_stack_commit: T,
    pub size_of_heap_reserve: T,
    pub size_of_heap_commit: T,
    pub _loader_flags: u32, // must be zero
    pub number_of_rva_and_sizes: u32,

    pub directories: [DataDirectory; DATA_DIRS_COUNT],
}

#[repr(u32)]
#[derive(EnumIter, Copy, Clone)]
pub enum SectionFlags {
    TypeNoPad = 0x00000008,
    ContainsCode = 0x00000020,
    ContainsInitializedData = 0x00000040,
    ContainsUnitializedData = 0x00000080,
    Info = 0x00000200,
    Remove = 0x00000800,
    COMDAT = 0x00001000,
    GPRelative = 0x00008000,
    Align1Byte = 0x00100000,
    Align2Bytes = 0x00200000,
    Align4Bytes = 0x00300000,
    Align8Bytes = 0x00400000,
    Align16Bytes = 0x00500000,
    Align32Bytes = 0x00600000,
    Align64Bytes = 0x00700000,
    Align128Bytes = 0x00800000,
    Align256Bytes = 0x00900000,
    Align512Bytes = 0x00A00000,
    Align1024Bytes = 0x00B00000,
    Align2048Bytes = 0x00C00000,
    Align4096Bytes = 0x00D00000,
    Align8192Bytes = 0x00E00000,
    NRelocOVFL = 0x01000000,
    MemDiscardable = 0x02000000,
    MemNotCached = 0x04000000,
    MemNotPaged = 0x08000000,
    MemShared = 0x10000000,
    MemExecute = 0x20000000,
    MemRead = 0x40000000,
    MemWrite = 0x80000000,
}
impl Into<u32> for SectionFlags {
    fn into(self) -> u32 {
        self as u32
    }
}

#[repr(C, packed)]
pub struct Section {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocation: u32,
    pub pointer_to_line_numbers: u32,
    pub number_of_relocations: u16,
    pub number_of_line_numbers: u16,
    pub characteristics: u32,
}

#[cfg(test)]
mod test {
    use crate::specification::*;

    #[test]
    fn size_check() {
        assert_eq!(std::mem::size_of::<MachineType>(), 2);
        assert_eq!(std::mem::size_of::<Characteristics>(), 2);
        assert_eq!(std::mem::size_of::<CoffHeader>(), 20);

        assert_eq!(std::mem::size_of::<OptionalHeadersStandard>(), 28);
        assert_eq!(std::mem::size_of::<OptionalHeadersWindows<u32>>(), 196);
        assert_eq!(std::mem::size_of::<OptionalHeadersWindows<u64>>(), 216);

        assert_eq!(std::mem::size_of::<DosHeader>(), 64);

        assert_eq!(std::mem::size_of::<Section>(), 40);
    }
}
