use clap::Parser;

#[derive(Parser)]
#[command(author,version,about,long_about = None)]
struct Args {
    /// Optional file arugment indicating the full path to your 'primsa.schema' file
    #[arg(long)]
    file: Option<String>,
}

use prismaviz::SchemaVisualiser;

fn main() {
    let args = Args::parse();
    match args.file {
        None => {
            panic!("Err!!!You forgot to pass a path to a file");
        }
        Some(v) => {
            let contents = std::fs::read_to_string(&v).unwrap();
            let visualiser = SchemaVisualiser::new(contents);
            visualiser.print_as_table();
        }
    }
}
