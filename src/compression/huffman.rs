use crate::compression::CompressionAlgorithm;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

// Node in the Huffman tree
#[derive(Debug, Clone)]
pub struct HuffmanNode {
    pub frequency: usize,
    pub byte: Option<u8>,  // None for internal nodes, Some(byte) for leaf nodes
    pub left: Option<Rc<RefCell<HuffmanNode>>>,
    pub right: Option<Rc<RefCell<HuffmanNode>>>,
}

impl HuffmanNode {
    // Create a new leaf node
    pub fn new_leaf(byte: u8, frequency: usize) -> Self {
        HuffmanNode {
            frequency,
            byte: Some(byte),
            left: None,
            right: None,
        }
    }
    
    // Create a new internal node
    pub fn new_internal(frequency: usize, left: Rc<RefCell<HuffmanNode>>, right: Rc<RefCell<HuffmanNode>>) -> Self {
        HuffmanNode {
            frequency,
            byte: None,
            left: Some(left),
            right: Some(right),
        }
    }
}

// Implement ordering traits for the priority queue
// want nodes with lower frequency to have higher priority (min heap)
impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering to make BinaryHeap behave like a min-heap
        other.frequency.cmp(&self.frequency)
    }
}

pub struct HuffmanCoding;

impl HuffmanCoding {
    // Build a frequency table for all bytes in the data
    fn build_frequency_table(data: &[u8]) -> HashMap<u8, usize> {
        let mut frequency_table = HashMap::new();
        
        for &byte in data {
            *frequency_table.entry(byte).or_insert(0) += 1;
        }
        
        frequency_table
    }

    //Build the Huffman tree from frequency table
    fn build_huffman_tree(frequency_table: &HashMap<u8, usize>) -> Option<Rc<RefCell<HuffmanNode>>> {
        if frequency_table.is_empty() {
            return None;
        }

        // Special case: only one unique byte
        if frequency_table.len() == 1 {
            let (&byte, &frequency) = frequency_table.iter().next().unwrap();
            return Some(Rc::new(RefCell::new(HuffmanNode::new_leaf(byte, frequency))));
        }

        // Create a priority queue (min-heap) and add all leaf nodes
        let mut heap = BinaryHeap::new();
        
        for (&byte, &frequency) in frequency_table {
            let leaf_node = HuffmanNode::new_leaf(byte, frequency);
            heap.push(Rc::new(RefCell::new(leaf_node)));
        }

        // Build the tree by combining nodes
        while heap.len() > 1 {
            // Take the two nodes with lowest frequency
            let right = heap.pop().unwrap();  
            let left = heap.pop().unwrap();
            
            // Calculate combined frequency
            let combined_frequency = left.borrow().frequency + right.borrow().frequency;
            
            // Create new internal node
            let internal_node = HuffmanNode::new_internal(combined_frequency, left, right);
            heap.push(Rc::new(RefCell::new(internal_node)));
        }

        // The last node is our root
        heap.pop()
    }

    // Generate Huffman codes from the tree
    fn generate_codes(root: &Rc<RefCell<HuffmanNode>>) -> HashMap<u8, String> {
        let mut codes = HashMap::new();
        
        // Handle the special case of only one unique byte
        let root_borrowed = root.borrow();
        if root_borrowed.byte.is_some() {
            // Only one unique byte - assign it code "0"
            if let Some(byte) = root_borrowed.byte {
                codes.insert(byte, "0".to_string());
            }
            return codes;
        }
        // Release the borrow before recursion
        drop(root_borrowed);

        // Recursively generate codes starting with empty string
        Self::generate_codes_recursive(root, String::new(), &mut codes);
        codes
    }

    // Recursive helper function to traverse the tree and build codes
    fn generate_codes_recursive(
        node: &Rc<RefCell<HuffmanNode>>, 
        current_code: String, 
        codes: &mut HashMap<u8, String>
    ) {
        let node_borrowed = node.borrow();
        
        // If this is a leaf node (has a byte value), save the code
        if let Some(byte) = node_borrowed.byte {
            codes.insert(byte, current_code);
            return;
        }
        
        // If this is an internal node, recurse on children
        if let Some(ref left) = node_borrowed.left {
            Self::generate_codes_recursive(left, current_code.clone() + "0", codes);
        }
        
        if let Some(ref right) = node_borrowed.right {
            Self::generate_codes_recursive(right, current_code + "1", codes);
        }
    }

    // Encode the data using the generated codes
    fn encode_data(data: &[u8], codes: &HashMap<u8, String>) -> Vec<u8> {
        // First, build a bit string for all our data
        let mut bit_string = String::new();
        
        for &byte in data {
            if let Some(code) = codes.get(&byte) {
                bit_string.push_str(code);
            }
        }
        
        println!("Total bits needed: {}", bit_string.len());
        println!("Original bytes: {}, Compressed bits: {} ({:.1}% of original)", 
                 data.len() * 8, bit_string.len(), 
                 (bit_string.len() as f64 / (data.len() * 8) as f64) * 100.0);
        
        // Now pack the bits into bytes
        Self::pack_bits_to_bytes(&bit_string)
    }

    // Helper function to pack a bit string into bytes
    fn pack_bits_to_bytes(bit_string: &str) -> Vec<u8> {
        let mut result = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;
        
        for bit_char in bit_string.chars() {
            // Shift the current byte left and add the new bit
            current_byte = (current_byte << 1) | if bit_char == '1' { 1 } else { 0 };
            bit_count += 1;
            
            // When we have 8 bits, save the byte and start a new one
            if bit_count == 8 {
                result.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }
        
        // Handle remaining bits (pad with zeros on the right)
        if bit_count > 0 {
            current_byte <<= 8 - bit_count; // Shift remaining bits to the left
            result.push(current_byte);
        }
        
        result
    }
}

impl CompressionAlgorithm for HuffmanCoding {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        // Build frequency table
        let frequency_table = Self::build_frequency_table(data);
        println!("Built frequency table with {} unique bytes", frequency_table.len());
        
        // Build Huffman tree
        let tree_root = Self::build_huffman_tree(&frequency_table);
        
        match tree_root {
            Some(root) => {
                println!("Successfully built Huffman tree with root frequency: {}", root.borrow().frequency);
                
                // Generate codes from the tree
                let codes = Self::generate_codes(&root);
                println!("Generated {} Huffman codes:", codes.len());
                
                // Let's print the codes to see what we got!
                for (byte, code) in &codes {
                    // Print the byte as a character if it's printable, otherwise as hex
                    let byte_display = if *byte >= 32 && *byte <= 126 {
                        format!("'{}'", *byte as char)
                    } else {
                        format!("0x{:02X}", byte)
                    };
                    println!("  {} -> {}", byte_display, code);
                }
                
                // Encode the actual data
                let compressed_data = Self::encode_data(data, &codes);
                println!("Compression successful! {} bytes -> {} bytes", data.len(), compressed_data.len());
                
                return Ok(compressed_data);
            },
            None => {
                return Ok(Vec::new());
            }
        }
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // TODO: Implement Huffman decompression
        Ok(data.to_vec())
    }
}
