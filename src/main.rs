use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'n', long = "name", required = true)]
    service_name: String,

    #[arg(short = 't', long = "type", required = true)]
    service_type: String,
}

fn main() {
    let args = Args::parse();

    println!("service_name: {}", args.service_name);
    println!("service_type: {}", args.service_type);

    return();
}
