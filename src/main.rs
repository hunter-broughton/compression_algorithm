use clap::{Arg, Command};
use std::fs;
use std::path::Path;

mod compression;

fn main() {
    let matches = Command::new("compression_algorithm")
        .version("0.1.0")
        .author("Your Name")
        .about("A compression algorithm implementation")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Input file to compress/decompress")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output file"))
        .arg(Arg::new("mode")
            .short('m')
            .long("mode")
            .value_name("MODE")
            .help("Mode: compress or decompress")
            .default_value("compress"))
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output");
    let mode = matches.get_one::<String>("mode").unwrap();

    println!("Compression Algorithm v0.1.0");
    println!("Input file: {}", input_file);
    println!("Mode: {}", mode);

    if !Path::new(input_file).exists() {
        eprintln!("Error: Input file '{}' does not exist", input_file);
        std::process::exit(1);
    }

    // TODO: Implement compression/decompression logic
    println!("Ready to implement your compression algorithm!");
}
