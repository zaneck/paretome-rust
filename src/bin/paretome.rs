use same_file::Handle;
use std::path::Path;

use clap::Parser;
use paretome_rust::pareto::add;

/// Command-line arguments for the GCD program.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The raw data to pareto sort.
    #[arg(short, long)]
    input: Option<String>,
    /// The pareto front.
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("Hello, Paretome! {}", add(2, 2));

    let input: Result<Handle, std::io::Error> = if args.input.is_none() {
        println!("No input file specified. Using stdin.");
        same_file::Handle::stdin()
    } else {
        println!("input file specified: {}", args.input.as_ref().unwrap());
        let input_path = Path::new(args.input.as_ref().unwrap().as_str());
        same_file::Handle::from_path(input_path)
    };

    let output: Result<Handle, std::io::Error> = if args.output.is_none() {
        println!("No output file specified. Using stdout.");
        same_file::Handle::stdout()
    } else {
        println!("output file specified: {}", args.output.as_ref().unwrap());
        let output_path = Path::new(args.input.as_ref().unwrap().as_str());
        same_file::Handle::from_path(output_path)
    };

    println!("input and output files: {:?}, {:?}", input, output);
}
