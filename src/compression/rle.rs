use crate::compression::CompressionAlgorithm;

pub struct RunLengthEncoding;

impl RunLengthEncoding {
    /// Encode a run of repeated bytes
    fn encode_run(output: &mut Vec<u8>, byte: u8, count: u8) {
        if count == 1 {
            // Single byte - check if it's the escape byte
            if byte == 0xFF {
                // Escape the 0xFF byte: 0xFF 0x00 0xFF
                output.push(0xFF);
                output.push(0x00);
                output.push(0xFF);
            } else {
                // Single byte, output as-is
                output.push(byte);
            }
        } else if count <= 3 && byte != 0xFF {
            // For short runs of non-escape bytes, it's more efficient to output literals
            for _ in 0..count {
                output.push(byte);
            }
        } else {
            // Run of 2 or more bytes, or any run of escape byte
            // Format: 0xFF <count> <byte>
            output.push(0xFF);
            output.push(count);
            output.push(byte);
        }
    }

    /// Decode the next token from the input
    fn decode_next(data: &[u8], pos: &mut usize) -> Result<(u8, u8), Box<dyn std::error::Error>> {
        if *pos >= data.len() {
            return Err("Unexpected end of input".into());
        }

        let first_byte = data[*pos];
        *pos += 1;

        if first_byte != 0xFF {
            // Regular byte
            return Ok((first_byte, 1));
        }

        // Escape sequence starting with 0xFF
        if *pos >= data.len() {
            return Err("Incomplete escape sequence".into());
        }

        let second_byte = data[*pos];
        *pos += 1;

        if second_byte == 0x00 {
            // Escaped 0xFF byte: 0xFF 0x00 0xFF
            if *pos >= data.len() {
                return Err("Incomplete escaped 0xFF sequence".into());
            }
            let third_byte = data[*pos];
            *pos += 1;
            if third_byte != 0xFF {
                return Err("Invalid escaped 0xFF sequence".into());
            }
            Ok((0xFF, 1))
        } else {
            // Run encoding: 0xFF <count> <byte>
            if *pos >= data.len() {
                return Err("Incomplete run sequence".into());
            }
            let byte = data[*pos];
            *pos += 1;
            Ok((byte, second_byte))
        }
    }
}

impl CompressionAlgorithm for RunLengthEncoding {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        println!("Starting RLE compression on {} bytes of data", data.len());

        let mut output = Vec::new();
        let mut i = 0;

        while i < data.len() {
            let current_byte = data[i];
            let mut count = 1u8;

            // Count consecutive identical bytes (max 255)
            while i + (count as usize) < data.len() && 
                  data[i + (count as usize)] == current_byte && 
                  count < 255 {
                count += 1;
            }

            // Encode this run
            Self::encode_run(&mut output, current_byte, count);

            i += count as usize;
        }

        let compression_ratio = (output.len() as f64 / data.len() as f64) * 100.0;
        println!("RLE compression completed!");
        println!("Original size: {} bytes", data.len());
        println!("Compressed size: {} bytes", output.len());
        println!("Compression ratio: {:.1}%", compression_ratio);
        println!("Space saved: {:.1}%", 100.0 - compression_ratio);

        Ok(output)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        println!("Starting RLE decompression on {} bytes of compressed data", data.len());

        let mut output = Vec::new();
        let mut pos = 0;

        while pos < data.len() {
            let (byte, count) = Self::decode_next(data, &mut pos)?;
            
            // Add the decoded bytes to output
            for _ in 0..count {
                output.push(byte);
            }
        }

        println!("RLE decompression completed!");
        println!("Decompressed {} bytes to {} bytes", data.len(), output.len());

        Ok(output)
    }
}
