pub fn render_nalu_count(count: usize) -> String {
    format!("nal_units={count}\n")
}

pub struct DumpLengthRow {
    pub nal_num: usize,
    pub frame_num: usize,
    pub nal_unit_type: u32,
    pub nal_unit_type_str: &'static str,
    pub nal_length_bytes: usize,
    pub first_slice_segment_in_pic_flag: Option<u32>,
    pub slice_segment_address: Option<u32>,
    pub slice_pic_order_cnt_lsb: Option<u32>,
}

pub struct DumpLengthFrameRow {
    pub frame_num: usize,
    pub nal_unit_type: u32,
    pub bitrate_bps: usize,
}

pub fn render_dump_length(rows: &[DumpLengthRow], frame_rows: &[DumpLengthFrameRow]) -> String {
    let mut output = String::new();
    output.push_str(
        "nal_num,frame_num,nal_unit_type,nal_unit_type_str,nal_length_bytes,bitrate_bps,first_slice_segment_in_pic_flag,slice_segment_address,slice_pic_order_cnt_lsb\n",
    );

    let mut frame_iter = frame_rows.iter().peekable();
    for row in rows {
        while let Some(frame_row) = frame_iter.peek() {
            if frame_row.frame_num != row.frame_num {
                break;
            }
            output.push_str(&format!(
                ",{},{},frame,,{},,,\n",
                frame_row.frame_num, frame_row.nal_unit_type, frame_row.bitrate_bps
            ));
            let _ = frame_iter.next();
        }

        output.push_str(&format!(
            "{},{},{},{},{},,{},{},{}\n",
            row.nal_num,
            row.frame_num,
            row.nal_unit_type,
            row.nal_unit_type_str,
            row.nal_length_bytes,
            row.first_slice_segment_in_pic_flag
                .map(|v| v.to_string())
                .unwrap_or_default(),
            row.slice_segment_address
                .map(|v| v.to_string())
                .unwrap_or_default(),
            row.slice_pic_order_cnt_lsb
                .map(|v| v.to_string())
                .unwrap_or_default(),
        ));
    }

    for frame_row in frame_iter {
        output.push_str(&format!(
            ",{},{},frame,,{},,,\n",
            frame_row.frame_num, frame_row.nal_unit_type, frame_row.bitrate_bps
        ));
    }

    output
}
