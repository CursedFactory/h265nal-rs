use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawSpsMultilayerExtensionFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpsMultilayerExtensionFields {
    pub inter_view_mv_vert_constraint_flag: u32,
}

/// Parses sps_multilayer_extension and returns scalar fields.
///
/// Native references:
/// - Declaration: `include/h265_sps_multilayer_extension_parser.h:44` (`ParseSpsMultilayerExtension`)
/// - Unit test: `test/h265_sps_multilayer_extension_parser_unittest.cc:21`
pub fn sps_multilayer_extension_parse(data: &[u8]) -> Result<SpsMultilayerExtensionFields, Error> {
    let mut raw = RawSpsMultilayerExtensionFields {
        inter_view_mv_vert_constraint_flag: 0,
    };
    let status =
        unsafe { ffi::h265nal_sps_multilayer_extension_parse(data.as_ptr(), data.len(), &mut raw) };
    status_to_result(status)?;
    Ok(SpsMultilayerExtensionFields {
        inter_view_mv_vert_constraint_flag: raw.inter_view_mv_vert_constraint_flag,
    })
}
