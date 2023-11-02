use crate::specification::*;
use crate::types::*;
use chrono::offset::Utc;
use chrono::DateTime;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("DOS Header magic value is wrong")]
    DosHeaderInvalidMagic,
    #[error("COFF Header magic value is wrong")]
    COFFHeaderInvalidMagic,
    #[error("Invalid COFF.time_stamp")]
    InvalidTimeStamp
}

unsafe fn parse_coff(coff: *const CoffHeader) -> Result<COFF, ParserError> {
    let coff = coff.as_ref().unwrap();
    let mut timestamp_seconds = DateTime::UNIX_EPOCH.timestamp() as u64;
    timestamp_seconds &= 0xffffffff00000000; // null the lsb part
    timestamp_seconds |= coff.time_date_stamp as u64;

    let timestamp = DateTime::<Utc>::from_timestamp(timestamp_seconds as i64, 0);
    Ok(COFF {
        machine: coff.machine,
        time_stamp: timestamp,
        characteristics: EnumSet::from(coff.characteristics),
    })
}
pub unsafe fn from_ptr(x: *const u8) -> Result<Image, ParserError> {
    let image = Image::default();

    let dos_ptr: *const DosHeader = std::mem::transmute(x);
    if (*dos_ptr).e_magic != DOS_HEADER_MAGIC {
        return Err(ParserError::DosHeaderInvalidMagic);
    }

    let coff_header = (x as usize + (*dos_ptr).e_lfanew as usize) as *const CoffHeader;
    if (*coff_header).magic != PE_HEADER_MAGIC {
        return Err(ParserError::COFFHeaderInvalidMagic);
    }

    let optional_standard = (coff_header as usize + std::mem::size_of::<CoffHeader>() as usize)
        as *const OptionalHeadersStandard;

    // here begins parsing logic
    let coff = parse_coff()

    Ok(image)
}
