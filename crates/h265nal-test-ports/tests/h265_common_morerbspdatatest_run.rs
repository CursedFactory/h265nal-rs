//! References:
//! - Markdown: `docs/test-cases/h265_common_unittest/h265commonmorerbspdatatest--run.md`
//! - C++: `test/h265_common_unittest.cc:129`
//! - Port note: Group 02 / Case 06

#[derive(Debug)]
struct TestCase {
    description: &'static str,
    buffer: &'static [u8],
    byte_offset: usize,
    bit_offset: usize,
    expected_result: bool,
}

const TEST_CASES: &[TestCase] = &[
    TestCase {
        description: "case 1: no more data in the bit buffer",
        buffer: &[0x00, 0x00],
        byte_offset: 2,
        bit_offset: 0,
        expected_result: false,
    },
    TestCase {
        description: "case 2: more than 1 byte left",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0xb0],
        byte_offset: 4,
        bit_offset: 4,
        expected_result: true,
    },
    TestCase {
        description: "case 3: at last byte (begin), with 1000,0000",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x80],
        byte_offset: 5,
        bit_offset: 0,
        expected_result: false,
    },
    TestCase {
        description: "case 4: at last byte (begin), with 1000,1000",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x88],
        byte_offset: 5,
        bit_offset: 0,
        expected_result: true,
    },
    TestCase {
        description: "case 5: at last byte (bit 1), with 1-100,0000",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0xc0],
        byte_offset: 5,
        bit_offset: 1,
        expected_result: false,
    },
    TestCase {
        description: "case 6: at last byte (bit 1), with 0-100,0000",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x40],
        byte_offset: 5,
        bit_offset: 1,
        expected_result: false,
    },
    TestCase {
        description: "case 7: at last byte (bit 1), with 0-000,0000 (no rbsp_trailing_bits())",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x00],
        byte_offset: 5,
        bit_offset: 1,
        expected_result: true,
    },
    TestCase {
        description: "case 8: at last byte (bit 1), with 0-100,1000",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x48],
        byte_offset: 5,
        bit_offset: 1,
        expected_result: true,
    },
    TestCase {
        description: "case 9: at last byte (bit 4), with 1-1000",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x18],
        byte_offset: 5,
        bit_offset: 4,
        expected_result: false,
    },
    TestCase {
        description: "case 10: at last byte (bit 4), with 0-1000",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x08],
        byte_offset: 5,
        bit_offset: 4,
        expected_result: false,
    },
    TestCase {
        description: "case 11: at last byte (bit 4), with 0000 (no rbsp_trailing_bits())",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x00],
        byte_offset: 5,
        bit_offset: 4,
        expected_result: true,
    },
    TestCase {
        description: "case 12: at last byte (bit 4), with 0010",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x02],
        byte_offset: 5,
        bit_offset: 4,
        expected_result: true,
    },
    TestCase {
        description: "case 13: at last byte (bit 7), with 1",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x01],
        byte_offset: 5,
        bit_offset: 7,
        expected_result: false,
    },
    TestCase {
        description: "case 14: at last byte (bit 7), with 0 (no rbsp_trailing_bits())",
        buffer: &[0xe8, 0x43, 0x82, 0x92, 0xc8, 0x00],
        byte_offset: 5,
        bit_offset: 7,
        expected_result: true,
    },
];

#[test]
fn run() {
    for testcase in TEST_CASES {
        let result = h265nal_sys::common_more_rbsp_data(
            testcase.buffer,
            testcase.byte_offset,
            testcase.bit_offset,
        )
        .unwrap_or_else(|err| panic!("{} failed: {err}", testcase.description));

        assert_eq!(
            result.has_more_data, testcase.expected_result,
            "{}",
            testcase.description
        );
        assert_eq!(
            result.byte_offset, testcase.byte_offset,
            "byte offset changed for {}",
            testcase.description
        );
        assert_eq!(
            result.bit_offset, testcase.bit_offset,
            "bit offset changed for {}",
            testcase.description
        );
    }
}
