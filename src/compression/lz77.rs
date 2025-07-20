use crate::compression::CompressionAlgorithm;

// LZ77 configuration constants
const WINDOW_SIZE: usize = 4096;    // Size of the sliding window (search buffer)
const LOOKAHEAD_SIZE: usize = 18;   // Size of the lookahead buffer
const MIN_MATCH_LENGTH: usize = 3;  // Minimum match length to be worth encoding

#[derive(Debug, Clone)]
struct Match {
    distance: u16,
    length: u16,
    next_char: u8,
}

pub struct LZ77;

impl LZ77 {
    /// Find the longest match in the search buffer for the lookahead buffer
    fn find_longest_match(data: &[u8], position: usize) -> Match {
        let mut best_match = Match {
            distance: 0,
            length: 0,
            next_char: if position < data.len() { data[position] } else { 0 },
        };

        // Define the search window bounds
        let search_start = if position >= WINDOW_SIZE { position - WINDOW_SIZE } else { 0 };
        let search_end = position;
        
        // Define the lookahead bounds
        let lookahead_start = position;
        let lookahead_end = std::cmp::min(position + LOOKAHEAD_SIZE, data.len());

        // Search for matches in the search buffer
        for search_pos in search_start..search_end {
            let mut match_length = 0;
            
            // Calculate how long the match is
            while search_pos + match_length < search_end &&
                  lookahead_start + match_length < lookahead_end &&
                  data[search_pos + match_length] == data[lookahead_start + match_length] {
                match_length += 1;
                
                // Prevent infinite matches by limiting to the distance
                if match_length >= position - search_pos {
                    break;
                }
            }

            // Update best match if this one is longer and meets minimum length
            if match_length >= MIN_MATCH_LENGTH && match_length > best_match.length as usize {
                best_match.distance = (position - search_pos) as u16;
                best_match.length = match_length as u16;
                best_match.next_char = if lookahead_start + match_length < data.len() {
                    data[lookahead_start + match_length]
                } else {
                    0
                };
            }
        }

        best_match
    }

    /// Encode a match or literal into the output buffer
    fn encode_token(output: &mut Vec<u8>, token: &Match) -> Result<(), Box<dyn std::error::Error>> {
        if token.length == 0 {
            // Literal byte - encode as: 0 + byte
            output.push(0x00); // Flag for literal
            output.push(token.next_char);
        } else {
            // Match - encode as: 1 + distance (2 bytes) + length (2 bytes) + next_char
            output.push(0x01); // Flag for match
            output.extend_from_slice(&token.distance.to_le_bytes());
            output.extend_from_slice(&token.length.to_le_bytes());
            output.push(token.next_char);
        }
        Ok(())
    }

    /// Decode a token from the input buffer
    fn decode_token(input: &[u8], pos: &mut usize) -> Result<Match, Box<dyn std::error::Error>> {
        if *pos >= input.len() {
            return Err("Unexpected end of input".into());
        }

        let flag = input[*pos];
        *pos += 1;

        if flag == 0x00 {
            // Literal
            if *pos >= input.len() {
                return Err("Unexpected end of input for literal".into());
            }
            let byte = input[*pos];
            *pos += 1;
            Ok(Match {
                distance: 0,
                length: 0,
                next_char: byte,
            })
        } else if flag == 0x01 {
            // Match
            if *pos + 4 >= input.len() {
                return Err("Unexpected end of input for match".into());
            }
            
            let distance = u16::from_le_bytes([input[*pos], input[*pos + 1]]);
            *pos += 2;
            let length = u16::from_le_bytes([input[*pos], input[*pos + 1]]);
            *pos += 2;
            let next_char = input[*pos];
            *pos += 1;
            
            Ok(Match {
                distance,
                length,
                next_char,
            })
        } else {
            Err("Invalid token flag".into())
        }
    }
}

impl CompressionAlgorithm for LZ77 {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        println!("Starting LZ77 compression on {} bytes of data", data.len());
        
        let mut output = Vec::new();
        let mut position = 0;
        
        // Add a simple header with original size for verification
        output.extend_from_slice(&(data.len() as u32).to_le_bytes());
        
        while position < data.len() {
            let token = Self::find_longest_match(data, position);
            
            Self::encode_token(&mut output, &token)?;
            
            // Move position forward
            if token.length == 0 {
                position += 1; // Just a literal
            } else {
                position += token.length as usize; // Skip the matched portion
                if position < data.len() {
                    position += 1; // Include the next character
                }
            }
        }

        let compression_ratio = (output.len() as f64 / data.len() as f64) * 100.0;
        println!("LZ77 compression completed!");
        println!("Original size: {} bytes", data.len());
        println!("Compressed size: {} bytes", output.len());
        println!("Compression ratio: {:.1}%", compression_ratio);
        println!("Space saved: {:.1}%", 100.0 - compression_ratio);

        Ok(output)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if data.len() < 4 {
            return Err("Invalid compressed data: too short".into());
        }

        println!("Starting LZ77 decompression on {} bytes of compressed data", data.len());
        
        // Read the original size from the header
        let original_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let mut output = Vec::with_capacity(original_size);
        let mut pos = 4; // Skip the header
        
        while pos < data.len() && output.len() < original_size {
            let token = Self::decode_token(data, &mut pos)?;
            
            if token.length == 0 {
                // Literal
                output.push(token.next_char);
            } else {
                // Match - copy from earlier in the output
                let start_pos = if output.len() >= token.distance as usize {
                    output.len() - token.distance as usize
                } else {
                    return Err("Invalid distance in compressed data".into());
                };
                
                // Copy the matched sequence
                for i in 0..token.length as usize {
                    if start_pos + i < output.len() {
                        let byte = output[start_pos + i];
                        output.push(byte);
                    } else {
                        return Err("Invalid match length in compressed data".into());
                    }
                }
                
                // Add the next character if we haven't reached the end
                if output.len() < original_size {
                    output.push(token.next_char);
                }
            }
        }

        // Truncate to the original size in case we went over
        output.truncate(original_size);
        
        println!("LZ77 decompression completed!");
        println!("Decompressed {} bytes to {} bytes", data.len(), output.len());
        
        Ok(output)
    }
}
