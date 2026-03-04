# h265nal C ABI Contract Notes

## Status Codes
- `H265NAL_STATUS_OK = 0`
- `H265NAL_STATUS_INVALID_ARGUMENT = 1`
- `H265NAL_STATUS_PARSE_FAILURE = 2`
- `H265NAL_STATUS_DUMP_UNAVAILABLE = 3`

## Ownership Rules
- ABI callers own all input buffers passed to `h265nal_*` symbols.
- Callers retain ownership of output scalars and pointers they pass in (for example `size_t* out_count`).
- No heap ownership transfer happens across the C ABI boundary in the current surface.
- ABI functions never require callers to free parser-owned objects.

## Versioning
- ABI version symbol: `h265nal_abi_version()`.
- Current ABI constant: `H265NAL_C_ABI_VERSION = 1`.
- Policy: additive changes only within ABI version `1` (no signature changes for existing symbols).
