from flask import Flask, request, jsonify, send_file
from flask_cors import CORS
import os
import tempfile
import subprocess
import json
from werkzeug.utils import secure_filename
import uuid
from datetime import datetime

app = Flask(__name__)
CORS(app)  # Enable CORS for all routes

# Configuration
UPLOAD_FOLDER = 'uploads'
COMPRESSED_FOLDER = 'compressed'
ALLOWED_EXTENSIONS = {'txt', 'csv', 'json', 'xml', 'html', 'css', 'js', 'py', 'rs', 'md', 'log'}

# Ensure directories exist
os.makedirs(UPLOAD_FOLDER, exist_ok=True)
os.makedirs(COMPRESSED_FOLDER, exist_ok=True)

def allowed_file(filename):
    return '.' in filename and \
           filename.rsplit('.', 1)[1].lower() in ALLOWED_EXTENSIONS

def compress_with_rust(input_data):
    """Call the Rust compression algorithm"""
    try:
        # Create a temporary input file
        with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as temp_input:
            temp_input.write(input_data)
            temp_input_path = temp_input.name
        
        # Create a temporary output file
        temp_output_path = temp_input_path + '.compressed'
        
        # Run the Rust compression algorithm
        result = subprocess.run([
            'cargo', 'run', '--', 
            '--input', temp_input_path,
            '--output', temp_output_path,
            '--mode', 'compress',
            '--algorithm', 'huffman'
        ], 
        cwd='/Users/hunterbroughton/compression_algorithm',
        capture_output=True, 
        text=True,
        timeout=30
        )
        
        print(f"Rust compression output: {result.stdout}")
        print(f"Rust compression errors: {result.stderr}")
        
        if result.returncode == 0:
            # Read the compressed data if output file was created
            compressed_data = None
            if os.path.exists(temp_output_path):
                with open(temp_output_path, 'rb') as f:
                    compressed_data = f.read()
                os.unlink(temp_output_path)
            else:
                # If no output file, use the original data (compression failed)
                compressed_data = input_data.encode('utf-8')
            
            # Parse the output to get compression statistics
            output_lines = result.stdout.strip().split('\n')
            stats = {
                'original_size': len(input_data.encode('utf-8')),
                'compressed_size': len(compressed_data),
                'compression_ratio': 0
            }
            
            # Try to parse compression stats from Rust output
            for line in output_lines:
                if 'Original size:' in line and 'bytes' in line:
                    try:
                        original_size = int(line.split('Original size:')[1].split('bytes')[0].strip())
                        stats['original_size'] = original_size
                    except:
                        pass
                elif 'Compressed size:' in line and 'bytes' in line:
                    try:
                        compressed_size = int(line.split('Compressed size:')[1].split('bytes')[0].strip())
                        stats['compressed_size'] = compressed_size
                    except:
                        pass
                elif 'Space saved:' in line and '%' in line:
                    try:
                        ratio = float(line.split('Space saved:')[1].split('%')[0].strip())
                        stats['compression_ratio'] = ratio
                    except:
                        pass
            
            # Calculate compression ratio if not found in output
            if stats['compression_ratio'] == 0 and stats['original_size'] > 0:
                stats['compression_ratio'] = ((stats['original_size'] - stats['compressed_size']) / stats['original_size']) * 100
            
            # Clean up temp input file
            os.unlink(temp_input_path)
            
            return {
                'success': True,
                'stats': stats,
                'compressed_data': compressed_data
            }
        else:
            # Clean up temp files
            os.unlink(temp_input_path)
            if os.path.exists(temp_output_path):
                os.unlink(temp_output_path)
            
            return {
                'success': False,
                'error': f"Compression failed: {result.stderr}"
            }
            
    except subprocess.TimeoutExpired:
        return {
            'success': False,
            'error': "Compression timed out"
        }
    except Exception as e:
        return {
            'success': False,
            'error': f"Error during compression: {str(e)}"
        }

@app.route('/api/upload-compress', methods=['POST'])
def upload_and_compress():
    if 'file' not in request.files:
        return jsonify({'error': 'No file provided'}), 400
    
    file = request.files['file']
    if file.filename == '':
        return jsonify({'error': 'No file selected'}), 400
    
    if not allowed_file(file.filename):
        return jsonify({'error': 'File type not allowed'}), 400
    
    try:
        # Read file content
        file_content = file.read().decode('utf-8')
        original_filename = secure_filename(file.filename)
        
        # Compress the content
        compression_result = compress_with_rust(file_content)
        
        if not compression_result['success']:
            return jsonify({'error': compression_result['error']}), 500
        
        # Generate unique filename for compressed file
        file_id = str(uuid.uuid4())
        compressed_filename = f"{file_id}_{original_filename}.compressed"
        compressed_path = os.path.join(COMPRESSED_FOLDER, compressed_filename)
        
        # Save compressed data
        with open(compressed_path, 'wb') as f:
            f.write(compression_result['compressed_data'])
        
        # Prepare response
        response_data = {
            'file_id': file_id,
            'original_filename': original_filename,
            'compressed_filename': compressed_filename,
            'original_size': len(file_content.encode('utf-8')),
            'compressed_size': len(compression_result['compressed_data']),
            'compression_ratio': compression_result['stats'].get('compression_ratio', 0),
            'timestamp': datetime.now().isoformat()
        }
        
        return jsonify(response_data)
        
    except UnicodeDecodeError:
        return jsonify({'error': 'File must be a text file'}), 400
    except Exception as e:
        return jsonify({'error': f'Processing failed: {str(e)}'}), 500

@app.route('/api/download/<file_id>')
def download_compressed_file(file_id):
    try:
        # Find the compressed file
        compressed_files = [f for f in os.listdir(COMPRESSED_FOLDER) if f.startswith(file_id)]
        
        if not compressed_files:
            return jsonify({'error': 'File not found'}), 404
        
        compressed_filename = compressed_files[0]
        compressed_path = os.path.join(COMPRESSED_FOLDER, compressed_filename)
        
        return send_file(
            compressed_path,
            as_attachment=True,
            download_name=compressed_filename,
            mimetype='application/octet-stream'
        )
        
    except Exception as e:
        return jsonify({'error': f'Download failed: {str(e)}'}), 500

@app.route('/api/compress-text', methods=['POST'])
def compress_text_endpoint():
    """Endpoint for text compression (existing functionality)"""
    try:
        data = request.get_json()
        text = data.get('text', '')
        
        if not text:
            return jsonify({'error': 'No text provided'}), 400
        
        compression_result = compress_with_rust(text)
        
        if not compression_result['success']:
            return jsonify({'error': compression_result['error']}), 500
        
        stats = compression_result['stats']
        return jsonify({
            'result': f"Original size: {stats['original_size']} bytes\n"
                     f"Compressed size: {stats['compressed_size']} bytes\n"
                     f"Reduction: {stats['compression_ratio']:.2f}%\n\n"
                     f"Compressed data (hex): {compression_result['compressed_data'].hex()}"
        })
        
    except Exception as e:
        return jsonify({'error': f'Compression failed: {str(e)}'}), 500

@app.route('/health')
def health_check():
    return jsonify({'status': 'healthy', 'timestamp': datetime.now().isoformat()})

@app.route('/')
def upload_page():
    """Serve the file upload HTML page"""
    return send_file('test_upload.html')

if __name__ == '__main__':
    print("Starting Flask server for file compression...")
    print(f"Upload folder: {os.path.abspath(UPLOAD_FOLDER)}")
    print(f"Compressed folder: {os.path.abspath(COMPRESSED_FOLDER)}")
    app.run(debug=True, port=5001)
