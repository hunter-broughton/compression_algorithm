#!/bin/bash

# Pied Piper File Compression Test Script
echo "Pied Piper File Compression Test"
echo "=================================="

if [ $# -eq 0 ]; then
    echo "Usage: $0 <file_to_compress>"
    echo "Example: $0 sample.txt"
    exit 1
fi

FILE_PATH="$1"

if [ ! -f "$FILE_PATH" ]; then
    echo "Error: File '$FILE_PATH' not found!"
    exit 1
fi

echo "Compressing file: $FILE_PATH"
echo ""

# Upload and compress the file
RESPONSE=$(curl -s -X POST -F "file=@$FILE_PATH" http://localhost:5001/api/upload-compress)

if [ $? -eq 0 ]; then
    echo "Compression successful!"
    echo ""
    echo "Results:"
    echo "$RESPONSE" | python3 -m json.tool
    
    # Extract file_id for download
    FILE_ID=$(echo "$RESPONSE" | python3 -c "import sys, json; print(json.load(sys.stdin)['file_id'])")
    
    echo ""
    echo "Download compressed file:"
    echo "curl -O http://localhost:5001/api/download/$FILE_ID"
    
    echo ""
    echo "Or use the web interface:"
    echo "file:///Users/hunterbroughton/compression_algorithm/file_server/test_upload.html"
else
    echo "Compression failed!"
fi
