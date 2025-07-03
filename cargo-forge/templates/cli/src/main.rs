use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: Option<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.count {
        match &cli.name {
            Some(name) => println!("Hello {}! Welcome to {{ project_name }}!", name),
            None => println!("Hello! Welcome to {{ project_name }}!"),
        }
    }
}
