use clap::Parser;

#[derive(Parser)]
#[command(author,version,about,long_about = None)]
struct Args {
    /// Optional file arugment indicating the full path to your 'primsa.schema' file
    #[arg(long)]
    file: Option<String>,
}

use prismaviz::parse_schema;

fn main() {
    let args = Args::parse();
    match args.file {
        None => {
            panic!("Err!!!You forgot to pass a path to a file");
        }
        Some(v) => {
            parse_schema(&v);
        }
    }
}
