pub mod huffman;
pub mod lz77;
pub mod rle;

pub trait CompressionAlgorithm {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}
