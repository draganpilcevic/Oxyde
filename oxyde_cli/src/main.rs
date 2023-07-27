use std::fs::File;

use clap::Parser;

/// Transpile a Rust Oxyde contract to 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The file to be converted to solidity
    pub input_file: String,
    // The path to the file to read
    //path: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();

    println!("received arg: {}", args.input_file);
    println!("things have CHANGED");
    println!("indeeeed");
    //println!("{:?}", args);

    File::create("temp/here.sol").unwrap();
    
}
