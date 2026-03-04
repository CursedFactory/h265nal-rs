//! Rust FFI bindings for the upstream h265nal native library.

mod annexb;
mod aud;
mod bitstream;
mod common;
mod configuration_box;
mod constants;
mod error;
mod ffi;
mod hrd_parameters;
mod nal_unit;
mod pps;
mod pps_multilayer_extension;
mod pps_scc_extension;
mod pred_weight_table;
mod profile_tier_level;
mod rtp;
mod scaling_list_data;
mod sei;
mod slice_segment_layer;
mod sps;
mod sps_3d_extension;
mod sps_multilayer_extension;
mod sps_range_extension;
mod sps_scc_extension;
mod st_ref_pic_set;
mod state;
mod sub_layer_hrd_parameters;
mod utils;
mod vps;
mod vui_parameters;

pub use annexb::{abi_version, count_nalus_annexb, dump_annexb_to_stdout};
pub use aud::aud_parse_pic_type;
pub use bitstream::{bitstream_parse, BitstreamNalFields};
pub use common::{
    common_is_nal_unit_type_non_vcl, common_is_nal_unit_type_unspecified,
    common_is_nal_unit_type_vcl, common_is_slice_segment, common_more_rbsp_data,
    MoreRbspDataResult,
};
pub use configuration_box::{configuration_box_parse, ConfigurationBoxFields};
pub use constants::{ABI_VERSION, DUMP_FLAG_ONE_LINE};
pub use error::Error;
pub use hrd_parameters::{hrd_parameters_parse, HrdParametersFields};
pub use nal_unit::{nal_unit_header_get_nal_unit_type, nal_unit_parse, NalUnitFields};
pub use pps::{pps_parse, PpsFields};
pub use pps_multilayer_extension::{pps_multilayer_extension_parse, PpsMultilayerExtensionFields};
pub use pps_scc_extension::{pps_scc_extension_parse, PpsSccExtensionFields};
pub use pred_weight_table::{pred_weight_table_parse, PredWeightTableFields};
pub use profile_tier_level::{profile_tier_level_parse, ProfileInfoFields, ProfileTierLevelFields};
pub use rtp::{
    rtp_parse, RtpFields, RTP_PACKET_KIND_AP, RTP_PACKET_KIND_FU, RTP_PACKET_KIND_NONE,
    RTP_PACKET_KIND_SINGLE,
};
pub use scaling_list_data::{scaling_list_data_parse, ScalingListDataFields};
pub use sei::{sei_parse, SeiMessageFields};
pub use slice_segment_layer::{slice_segment_layer_parse, SliceSegmentLayerFields};
pub use sps::{sps_parse, SpsFields};
pub use sps_3d_extension::{sps_3d_extension_parse, Sps3dExtensionFields};
pub use sps_multilayer_extension::{sps_multilayer_extension_parse, SpsMultilayerExtensionFields};
pub use sps_range_extension::{sps_range_extension_parse, SpsRangeExtensionFields};
pub use sps_scc_extension::{sps_scc_extension_parse, SpsSccExtensionFields};
pub use st_ref_pic_set::{st_ref_pic_set_parse, StRefPicSetFields};
pub use state::BitstreamParserState;
pub use sub_layer_hrd_parameters::{sub_layer_hrd_parameters_parse, SubLayerHrdParametersFields};
pub use utils::utils_get_slice_qp_y;
pub use vps::{vps_parse, VpsFields};
pub use vui_parameters::{vui_parameters_parse, VuiParametersFields};
