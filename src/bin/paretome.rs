use paretome_rust::pareto::add;
use structopt::StructOpt;

/// Command-line arguments for the GCD program.
#[derive(StructOpt)]
struct Cli {
    /// The raw data to pareto sort.
    input: String,
    /// The pareto front.
    output: String,
}

fn main(){
    let args = Cli::from_args();
    println!("Hello, Paretome! {}", add(2, 2));
    println!("input and output files: {}, {}", args.input, args.output);
}