//! References:
//! - Markdown: `docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpyiframebroken.md`
//! - C++: `test/h265_utils_unittest.cc:74`

#[test]
fn test_get_slice_qp_y_i_frame_broken() {
    let buffer: &[u8] = &[
        // slice (I-frame)
        0x00, 0x00, 0x00, 0x01, 0x26, 0x01, 0xaf,
    ];

    let mut state = h265nal_sys::BitstreamParserState::new().expect("state create failed");

    state.seed_vps(0).expect("seed_vps failed");
    state
        .seed_sps(0, 1, 1, 0, 0, 0, 0)
        .expect("seed_sps failed");
    state.seed_pps(0).expect("seed_pps failed");

    let slice_qp_y_vector =
        h265nal_sys::utils_get_slice_qp_y(buffer, &mut state).expect("get_slice_qp_y failed");

    // check there are no QP_Y values
    assert_eq!(slice_qp_y_vector.len(), 0);
}
