# resized

Batch image resizer with two modes: proportional fit and Instagram-ready canvas sizing.

## Installation


### Install script (macOS & Linux)

The quickest way to get `resized` is the install script, which detects your OS
(macOS or Linux) and architecture (Apple Silicon / ARM64 or Intel / x86_64),
downloads the latest prebuilt binary, and installs it to `/usr/local/bin`:

```bash
curl -fsSL https://raw.githubusercontent.com/kgantsov/resized/main/install.sh | bash
```

Or download and run it manually if you'd rather inspect it first:

```bash
curl -fsSLO https://raw.githubusercontent.com/kgantsov/resized/main/install.sh
bash install.sh
```

The script may prompt for `sudo` to write to `/usr/local/bin`. Once it finishes,
run `resized --help` to verify.

### Build from source

```bash
cargo build --release
# binary at target/release/resized
```

## Usage

```
resized [OPTIONS] <COMMAND>

Options:
  -i, --input-path <DIR>          Directory of input images [default: .]
  -o, --output-path <DIR>         Directory to write output images [default: .]
  -b, --border <PX>               Border in pixels on each side [default: 0]
  -b, --border-color <COLOR>      Border color: white or black [default: white]

Commands:
  fit        Proportional resize to fit within max dimensions
  instagram  Smart Instagram sizing
```

### `fit` — proportional resize

Scales images down to fit within `--max-width` × `--max-height` while preserving aspect ratio. Images smaller than the target are never upscaled. With `--border`, a margin is added around the resized image (color controlled by `--border-color`).

```bash
resized fit \
  --input-path  photos/raw/ \
  --output-path photos/web/ \
  --border 0 \
  --max-width 1920 \
  --max-height 1080
```

### `instagram` — canvas sizing

Fits each image onto a canvas sized for Instagram (color controlled by `--border-color`):
- **Portrait** (height > width) → 1080 × 1350
- **Landscape / square** → 1080 × 1080

The image is centered. `--border` adds extra padding inside the canvas edge.

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
