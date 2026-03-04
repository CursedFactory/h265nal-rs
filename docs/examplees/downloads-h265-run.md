# Downloads Sample Run

Input file:

`/Users/alexander.carter/Downloads/20260304-012633__p2-droid-1106__droid__center.h265`

Command used:

```bash
cargo run -q -p h265nal-cli -- \
  --infile "/Users/alexander.carter/Downloads/20260304-012633__p2-droid-1106__droid__center.h265" \
  --output-format pretty
```

Captured stdout file:

`docs/examplees/downloads_center.pretty.out`

Captured stdout:

```text
Input: /Users/alexander.carter/Downloads/20260304-012633__p2-droid-1106__droid__center.h265
NAL units: 2695
Mode: count
Dump mode: false
One-line: true
```
