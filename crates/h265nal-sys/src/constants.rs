pub(crate) const H265NAL_STATUS_OK: i32 = 0;
pub(crate) const H265NAL_STATUS_INVALID_ARGUMENT: i32 = 1;
pub(crate) const H265NAL_STATUS_PARSE_FAILURE: i32 = 2;
pub(crate) const H265NAL_STATUS_DUMP_UNAVAILABLE: i32 = 3;

pub const ABI_VERSION: u32 = 1;
pub const DUMP_FLAG_ONE_LINE: u32 = 1u32 << 0;
pub const DUMP_FLAG_ADD_OFFSET: u32 = 1u32 << 1;
pub const DUMP_FLAG_ADD_LENGTH: u32 = 1u32 << 2;
pub const DUMP_FLAG_ADD_PARSED_LENGTH: u32 = 1u32 << 3;
pub const DUMP_FLAG_ADD_CHECKSUM: u32 = 1u32 << 4;
pub const DUMP_FLAG_ADD_RESOLUTION: u32 = 1u32 << 5;
