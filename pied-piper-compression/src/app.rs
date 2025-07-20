use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use web_sys;

#[cfg(feature = "ssr")]
use compression_algorithm::compression::{huffman::HuffmanCoding, CompressionAlgorithm};

#[server(CompressText, "/api")]
pub async fn compress_text(text: String) -> Result<String, ServerFnError> {
    use compression_algorithm::compression::{huffman::HuffmanCoding, CompressionAlgorithm};
    
    let huffman = HuffmanCoding;
    let input_bytes = text.into_bytes();
    let input_size = input_bytes.len();
    
    match huffman.compress(&input_bytes) {
        Ok(compressed) => {
            let compressed_size = compressed.len();
            let reduction_ratio = if input_size > 0 {
                ((input_size as f64 - compressed_size as f64) / input_size as f64) * 100.0
            } else {
                0.0
            };
            
            Ok(format!(
                "Original size: {} bytes\nCompressed size: {} bytes\nReduction: {:.2}%\n\nCompressed data (hex): {}",
                input_size,
                compressed_size,
                reduction_ratio,
                compressed.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
            ))
        }
        Err(e) => Err(ServerFnError::ServerError(format!("Compression failed: {}", e))),
    }
}

#[server(UploadAndCompress, "/api")]
pub async fn upload_and_compress(file_data: String, filename: String) -> Result<String, ServerFnError> {
    // This will be handled by our Python Flask server
    // For now, we'll return a message indicating to use the file upload interface
    Ok(format!("File upload should be handled by the Flask server at http://localhost:5000/api/upload-compress"))
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pied-piper-compression.css"/>

        // sets the document title
        <Title text="Pied Piper - Advanced Compression"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // State for text compression
    let compression_result = RwSignal::new(None::<String>);
    let file_content = RwSignal::new(String::new());
    let is_processing = RwSignal::new(false);

    view! {
        <div class="cyberpunk-container">
            // Header with Pied Piper branding
            <header class="header">
                <div class="logo">
                    <h1 class="title-glitch" data-text="PIED PIPER">
                        "PIED PIPER"
                    </h1>
                    <p class="subtitle">"ADVANCED HUFFMAN COMPRESSION"</p>
                </div>
            </header>

            // Main compression interface
            <section class="main-interface">
                <div class="terminal-window">
                    <div class="terminal-header">
                        <span class="terminal-title">"COMPRESSION TERMINAL"</span>
                        <div class="terminal-controls">
                            <div class="control-dot red"></div>
                            <div class="control-dot yellow"></div>
                            <div class="control-dot green"></div>
                        </div>
                    </div>
                    
                    <div class="terminal-body">
                        <div class="input-section">
                            <label class="label">"INPUT DATA:"</label>
                            <textarea 
                                class="terminal-input"
                                placeholder="Enter text to compress..."
                                prop:value={move || file_content.get()}
                                on:input=move |ev| {
                                    file_content.set(event_target_value(&ev));
                                }
                            ></textarea>
                        </div>

                        <div class="controls">
                            <button 
                                class="btn-primary"
                                class:processing={move || is_processing.get()}
                                on:click=move |_| {
                                    let content = file_content.get();
                                    if content.trim().is_empty() {
                                        compression_result.set(Some("Error: Please enter some text to compress!".to_string()));
                                        return;
                                    }
                                    
                                    is_processing.set(true);
                                    
                                    // Call the server function to compress the text
                                    spawn_local(async move {
                                        match compress_text(content).await {
                                            Ok(result) => {
                                                compression_result.set(Some(result));
                                            }
                                            Err(e) => {
                                                compression_result.set(Some(format!("Error: {}", e)));
                                            }
                                        }
                                        is_processing.set(false);
                                    });
                                }
                            >
                                {move || if is_processing.get() { "PROCESSING..." } else { "COMPRESS TEXT" }}
                            </button>
                        </div>

                        {move || compression_result.get().map(|result| view! {
                            <div class="output-section">
                                <label class="label">"COMPRESSION RESULT:"</label>
                                <div class="result-display">
                                    <pre class="result-text">{result}</pre>
                                </div>
                            </div>
                        })}
                        
                        <div class="file-upload-info">
                            <h3>"FILE COMPRESSION AVAILABLE!"</h3>
                            <p>"For file uploads and downloads, use our dedicated interface:"</p>
                            <a href="http://localhost:5001/" 
                               target="_blank" 
                               class="btn-secondary">
                                "🗂️ OPEN FILE COMPRESSION TOOL"
                            </a>
                        </div>
                    </div>
                </div>

                // Stats display
                <div class="stats-panel">
                    <h3 class="stats-title">"SYSTEM STATUS"</h3>
                    <div class="stat-item">
                        <span class="stat-label">"ALGORITHM:"</span>
                        <span class="stat-value">"HUFFMAN CODING"</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"STATUS:"</span>
                        <span class="stat-value online">"ONLINE"</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"EFFICIENCY:"</span>
                        <span class="stat-value">"OPTIMAL"</span>
                    </div>
                </div>
            </section>

            // Footer
            <footer class="footer">
                <p>"POWERED BY RUST + WEBASSEMBLY"</p>
                <p class="copyright">"© 2025 PIED PIPER TECHNOLOGIES"</p>
            </footer>
        </div>
    }
}
