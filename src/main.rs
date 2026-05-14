use clap::Parser;
use resized::command::Cli;
use resized::processing::process_images;
use resized::walk::collect_image_paths;

fn main() {
    let cli = Cli::parse();

    if !cli.input_path.is_dir() {
        eprintln!(
            "error: input path does not exist or is not a directory: {:?}",
            cli.input_path
        );
        std::process::exit(1);
    }
    if !cli.output_path.is_dir() {
        eprintln!(
            "error: output path does not exist or is not a directory: {:?}",
            cli.output_path
        );
        std::process::exit(1);
    }

    let images_to_process = collect_image_paths(&cli.input_path);

    println!(
        "Found {} images in {:?}. Processing all of them.",
        images_to_process.len(),
        cli.input_path
    );

    let processed_images = process_images(
        cli.mode,
        cli.output_path.clone(),
        cli.border,
        images_to_process.clone(),
    );

    println!(
        "Done resizing {} images in {:?}",
        processed_images, cli.input_path
    );
}
