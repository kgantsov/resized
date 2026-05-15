use clap::Parser;
use resized::command::Cli;
use resized::processing::process_images;
use resized::walk::collect_image_paths;

fn main() {
    let cli = Cli::parse();

    let (images_to_process, output_is_file) = if cli.input_path.is_file() {
        // file-to-file or file-to-dir
        let output_is_file = !cli.output_path.is_dir();
        (vec![cli.input_path.clone()], output_is_file)
    } else if cli.input_path.is_dir() {
        if !cli.output_path.is_dir() {
            eprintln!(
                "error: output path does not exist or is not a directory: {:?}",
                cli.output_path
            );
            std::process::exit(1);
        }
        (collect_image_paths(&cli.input_path), false)
    } else {
        eprintln!(
            "error: input path does not exist or is not a file/directory: {:?}",
            cli.input_path
        );
        std::process::exit(1);
    };

    println!(
        "Found {} image(s). Processing.",
        images_to_process.len(),
    );

    let processed_images = process_images(
        cli.mode,
        cli.output_path.clone(),
        cli.border,
        cli.border_color,
        images_to_process,
        output_is_file,
    );

    println!("Done resizing {} image(s).", processed_images);
}
