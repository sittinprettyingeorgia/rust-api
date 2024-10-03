use clap::Parser;

/// Simple program to greet a person
/// run with 'cargo run -- --name John --count 3'
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
        println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
    }
}