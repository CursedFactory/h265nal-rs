//! References:
//! - Markdown: `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testmasteringdisplaycolourvolumesei.md`
//! - C++: `test/h265_sei_parser_unittest.cc:96`
//! - Port note: Group 07 / Case 35

#[test]
fn test_mastering_display_colour_volume_sei() {
    // Test data for mastering display colour volume SEI
    // This represents typical HDR10 metadata values
    let buffer = [
        0x89, 0x18, // payload_type = 137 (mastering_display_colour_volume), payload_size = 24
        0x84, 0x49, // display_primaries[0]_x = 0x8449 (33865 -> 0.6773 in CIE xy)
        0x7d, 0x00, // display_primaries[0]_y = 0x7d00 (32000 -> 0.64 in CIE xy)
        0x33, 0x5c, // display_primaries[1]_x = 0x335c (13148 -> 0.26296 in CIE xy)
        0xa9, 0xc2, // display_primaries[1]_y = 0xa9c2 (43458 -> 0.86916 in CIE xy)
        0x1d, 0x4c, // display_primaries[2]_x = 0x1d4c (7500 -> 0.15 in CIE xy)
        0x0b, 0xb8, // display_primaries[2]_y = 0x0bb8 (3000 -> 0.06 in CIE xy)
        0x3e, 0x80, // white_point_x = 0x3e80 (16000 -> 0.32 in CIE xy)
        0x46, 0x50, // white_point_y = 0x4650 (18000 -> 0.36 in CIE xy)
        0x00, 0x98, 0x96, 0x80, // max_display_mastering_luminance = 10000000 (1000 cd/m^2)
        0x00, 0x00, 0x00, 0x32, // min_display_mastering_luminance = 50 (0.005 cd/m^2)
    ];

    let sei = h265nal_sys::sei_parse(&buffer).expect("ParseSei failed");
    assert_eq!(sei.payload_type, 137);
    assert_eq!(sei.payload_size, 24);
    assert_eq!(sei.has_mastering_display_colour_volume, 1);
    assert_eq!(sei.mastering_display_display_primaries_x[0], 0x8449);
    assert_eq!(sei.mastering_display_display_primaries_y[0], 0x7d00);
    assert_eq!(sei.mastering_display_display_primaries_x[1], 0x335c);
    assert_eq!(sei.mastering_display_display_primaries_y[1], 0xa9c2);
    assert_eq!(sei.mastering_display_display_primaries_x[2], 0x1d4c);
    assert_eq!(sei.mastering_display_display_primaries_y[2], 0x0bb8);
    assert_eq!(sei.mastering_display_white_point_x, 0x3e80);
    assert_eq!(sei.mastering_display_white_point_y, 0x4650);
    assert_eq!(
        sei.mastering_display_max_display_mastering_luminance,
        10000000
    );
    assert_eq!(sei.mastering_display_min_display_mastering_luminance, 50);
}
