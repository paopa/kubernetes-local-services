use clap::Parser;

#[derive(Parser, Debug)]
#[command(
author = "paooap",
version = "0.0.1-beta",
// this one is working when you run wih -h
about = "TBD",
// this one is working when you run with --help
long_about = "This is a tool for easy to create a service in kubernetes."
)]
struct Args {
    /// The name of the service
    #[arg(short = 'n', long, required = true)]
    service_name: String,

    /// The type of the service
    #[arg(short = 't', long, required = true)]
    service_type: String,
}

fn main() {
    let args = Args::parse();

    println!("service_name: {}", args.service_name);
    println!("service_type: {}", args.service_type);

    return ();
}
