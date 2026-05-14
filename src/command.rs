use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Proportional resize to fit within max dimensions, no canvas padding
    Fit {
        /// Maximum output width in pixels (defaults to original width)
        #[arg(long)]
        max_width: Option<u32>,
        /// Maximum output height in pixels (defaults to original height)
        #[arg(long)]
        max_height: Option<u32>,
    },
    /// Smart Instagram sizing: 1080×1080 for landscape, 1080×1350 for portrait
    Instagram,
}

#[derive(Parser, Debug)]
#[command(
    name = "resized",
    about = "A simple image resizing tool with optional white borders."
)]
pub struct Cli {
    /// Directory of input images
    #[arg(short, long, default_value = ".", global = true)]
    pub input_path: PathBuf,

    /// Directory to write output images
    #[arg(short, long, default_value = ".", global = true)]
    pub output_path: PathBuf,

    /// Minimum white border in pixels on each side (default: 0)
    #[arg(short, long, default_value_t = 0, global = true)]
    pub border: u32,

    #[command(subcommand)]
    pub mode: Mode,
}
