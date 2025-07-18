# Compression Algorithm

A Rust implementation of various compression algorithms.

## Features

- Run Length Encoding (RLE)
- Huffman Coding
- LZ77 Algorithm
- Command-line interface for easy usage

## Usage

```bash

# Compress a file
cargo run -- -i input.txt -o compressed.bin -m compress

# Decompress a file
cargo run -- -i compressed.bin -o output.txt -m decompress
```

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## Algorithms to Implement

1. **Run Length Encoding (RLE)** - Simple compression for data with repeated sequences
2. **Huffman Coding** - Variable-length prefix coding for optimal compression
3. **LZ77** - Dictionary-based compression using sliding window

Each algorithm is implemented as a separate module with a common trait interface.
