//! References:
//! - Markdown: `docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testsamplebitstreamalt.md`
//! - C++: `test/h265_bitstream_parser_unittest.cc:135`
//! - Port note: Group 01 / Case 05

#[test]
#[ignore = "TODO: missing ParseBitstream metadata fields (offset/length/parsed_length/checksum)"]
fn test_sample_bitstream_alt() {
    // Keep this test ignored until parsed NAL metadata is exposed in Rust output.
}
