#![allow(dead_code)]

pub const TRAIL_N: u32 = 0;
pub const BLA_W_LP: u32 = 16;
pub const VPS_NUT: u32 = 32;
pub const RSV_NVCL43: u32 = 43;
pub const RSV_NVCL47: u32 = 47;
pub const UNSPEC50: u32 = 50;
pub const UNSPEC63: u32 = 63;

pub fn assert_bool_result(label: &str, actual: Result<bool, h265nal_sys::Error>, expected: bool) {
    let actual = actual.unwrap_or_else(|err| panic!("{label} failed: {err}"));
    assert_eq!(actual, expected, "{label}");
}
