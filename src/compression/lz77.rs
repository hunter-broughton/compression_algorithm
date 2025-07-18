use crate::compression::CompressionAlgorithm;

pub struct LZ77;

impl CompressionAlgorithm for LZ77 {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // TODO: Implement LZ77 compression
        // This involves finding matches in a sliding window and encoding them
        // as (distance, length, next_char) tuples
        println!("LZ77 compression not yet implemented");
        Ok(data.to_vec())
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // TODO: Implement LZ77 decompression
        println!("LZ77 decompression not yet implemented");
        Ok(data.to_vec())
    }
}
