# resized

Batch image resizer with two modes: proportional fit and Instagram-ready canvas sizing.

## Installation

```bash
cargo build --release
# binary at target/release/resized
```

## Usage

```
resized [OPTIONS] <COMMAND>

Options:
  -i, --input-path <DIR>   Directory of input images [default: .]
  -o, --output-path <DIR>  Directory to write output images [default: .]
  -b, --border <PX>        White border in pixels on each side [default: 0]

Commands:
  fit        Proportional resize to fit within max dimensions
  instagram  Smart Instagram sizing
```

### `fit` — proportional resize

Scales images down to fit within `--max-width` × `--max-height` while preserving aspect ratio. Images smaller than the target are never upscaled. With `--border`, a white margin is added around the resized image.

```bash
resized fit \
  --input-path  photos/raw/ \
  --output-path photos/web/ \
  --border 0 \
  --max-width 1920 \
  --max-height 1080
```

### `instagram` — canvas sizing

Fits each image onto a white canvas sized for Instagram:
- **Portrait** (height > width) → 1080 × 1350
- **Landscape / square** → 1080 × 1080

The image is centered. `--border` adds extra white space inside the canvas edge.

```bash
resized instagram \
  --input-path  photos/raw/ \
  --output-path photos/instagram/ \
  --border 40
```

## Details

- Processes images in parallel (rayon)
- Resamples with Lanczos3 filter
- Output format inferred from file extension (`jpg`, `png`, `webp`, etc.)
- Non-image files in the input directory are silently skipped
- Supported input formats: JPEG, PNG, GIF, BMP, WebP, TIFF

## Development

```bash
cargo build   # compile
cargo test    # run tests
```
