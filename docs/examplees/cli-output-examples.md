# h265nal-cli Examples

## Example 1: C output with full dump

Command:

```bash
cargo run -q -p h265nal-cli -- \
  --infile video/akiyo.x265.qp_30.265 \
  --dump-all \
  --no-as-one-line \
  --add-offset \
  --add-length \
  --output-format c
```

Captured full output: `docs/examplees/akiyo_qp30_dump.c.out`

Excerpt:

```text
nal_unit {
  offset: 0x00000004
  length: 24
  parsed_length: 21
  checksum: 0xb6c35c75
  nal_unit_header {
    forbidden_zero_bit: 0
    nal_unit_type: 32
    nuh_layer_id: 0
    nuh_temporal_id_plus1: 1
  }
  nal_unit_payload {
    vps {
      vps_video_parameter_set_id: 0
      vps_base_layer_internal_flag: 1
      vps_base_layer_available_flag: 1
      vps_max_layers_minus1: 0
      vps_max_sub_layers_minus1: 0
      vps_temporal_id_nesting_flag: 1
```

## Example 2: Pretty summary output

Command:

```bash
cargo run -q -p h265nal-cli -- \
  --infile video/iphone_11s.messenger.704x1280.265 \
  --output-format pretty
```

Captured output: `docs/examplees/iphone_pretty.out`

```text
Input: video/iphone_11s.messenger.704x1280.265
NAL units: 254
Mode: count
Dump mode: false
One-line: true
```

## Example 3: Simple count in C mode

Command:

```bash
cargo run -q -p h265nal-cli -- \
  --infile video/akiyo.x265.qp_30.265 \
  --output-format c \
  --add-offset \
  --add-length
```

Captured output: `docs/examplees/akiyo_qp30.c.out`

```text
nal_units=308
```
