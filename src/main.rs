use clap::{Arg, Command};
use std::fs;
use std::path::Path;

mod compression;
use compression::CompressionAlgorithm;

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
        .arg(Arg::new("algorithm")
            .short('a')
            .long("algorithm")
            .value_name("ALGORITHM")
            .help("Compression algorithm: huffman, lz77, or rle")
            .default_value("huffman"))
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output");
    let mode = matches.get_one::<String>("mode").unwrap();
    let algorithm = matches.get_one::<String>("algorithm").unwrap();

    println!("Compression Algorithm v0.1.0");
    println!("Input file: {}", input_file);
    println!("Mode: {}", mode);
    println!("Algorithm: {}", algorithm);

    if !Path::new(input_file).exists() {
        eprintln!("Error: Input file '{}' does not exist", input_file);
        std::process::exit(1);
    }

    // Read the input file
    let data = match fs::read(input_file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            std::process::exit(1);
        }
    };

    // Select the compression algorithm
    let compressor: Box<dyn CompressionAlgorithm> = match algorithm.as_str() {
        "huffman" => Box::new(compression::huffman::HuffmanCoding),
        "lz77" => Box::new(compression::lz77::LZ77),
        "rle" => Box::new(compression::rle::RunLengthEncoding),
        _ => {
            eprintln!("Unknown algorithm: {}. Available algorithms: huffman, lz77, rle", algorithm);
            std::process::exit(1);
        }
    };
    
    match mode.as_str() {
        "compress" => {
            println!("Starting {} compression on {} bytes of data", algorithm, data.len());
            
            // Show a preview of the text if it's printable
            if data.len() <= 100 && data.iter().all(|&b| b >= 32 && b <= 126 || b == b'\n' || b == b'\r' || b == b'\t') {
                let text = String::from_utf8_lossy(&data);
                println!("Text content: {:?}", text);
            }
            
            match compressor.compress(&data) {
                Ok(compressed) => {
                    println!("Compression test completed!");
                    
                    // Save to output file if specified
                    if let Some(output_path) = output_file {
                        match fs::write(output_path, &compressed) {
                            Ok(()) => {
                                println!("Compressed data saved to '{}'", output_path);
                                println!("Original size: {} bytes", data.len());
                                println!("Compressed size: {} bytes", compressed.len());
                                let ratio = (compressed.len() as f64 / data.len() as f64) * 100.0;
                                println!("Compression ratio: {:.1}%", ratio);
                                println!("Space saved: {:.1}%", 100.0 - ratio);
                            },
                            Err(e) => {
                                eprintln!("Error writing compressed file '{}': {}", output_path, e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        println!("No output file specified. Use -o to save compressed data.");
                        println!("Compressed data size: {} bytes", compressed.len());
                    }
                },
                Err(e) => eprintln!("Compression failed: {}", e),
            }
        },
        "decompress" => {
            println!("Starting {} decompression on {} bytes of data", algorithm, data.len());
            
            match compressor.decompress(&data) {
                Ok(decompressed) => {
                    println!("Decompression test completed!");
                    
                    // Save to output file if specified
                    if let Some(output_path) = output_file {
                        match fs::write(output_path, &decompressed) {
                            Ok(()) => {
                                println!("Decompressed data saved to '{}'", output_path);
                                println!("Compressed size: {} bytes", data.len());
                                println!("Decompressed size: {} bytes", decompressed.len());
                            },
                            Err(e) => {
                                eprintln!("Error writing decompressed file '{}': {}", output_path, e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        println!("No output file specified. Use -o to save decompressed data.");
                        println!("Decompressed data size: {} bytes", decompressed.len());
                    }
                },
                Err(e) => eprintln!("Decompression failed: {}", e),
            }
        },
        _ => {
            eprintln!("Invalid mode: {}. Use 'compress' or 'decompress'", mode);
            std::process::exit(1);
        }
    }
}
