use same_file::Handle;
use std::path::Path;

use paretome_rust::pareto::add;
use structopt::StructOpt;

/// Command-line arguments for the GCD program.
#[derive(StructOpt)]
struct Cli {
    /// The raw data to pareto sort.
    input: Option<String>,
    /// The pareto front.
    output: Option<String>,
}

fn main(){
    let args = Cli::from_args();
    println!("Hello, Paretome! {}", add(2, 2));

    let input: Result<Handle, std::io::Error> = if args.input.is_none(){
        same_file::Handle::stdin()
    }
    else {
        let input_path = Path::new(args.input.as_ref().unwrap().as_str());
        same_file::Handle::from_path(input_path)
    };

    let output: Result<Handle, std::io::Error> = if args.output.is_none(){
        same_file::Handle::stdout()
    }
    else {
        let output_path = Path::new(args.input.as_ref().unwrap().as_str());
        same_file::Handle::from_path(output_path)
    };

    println!("input and output files: {:?}, {:?}", input, output);
}