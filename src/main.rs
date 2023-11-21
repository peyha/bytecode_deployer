use clap::Parser;

#[derive(Parser)]
struct Cli {
    bytecode: String,
}

fn main() {
    let args = Cli::parse();
    
    println!("Hello, world!");
}
