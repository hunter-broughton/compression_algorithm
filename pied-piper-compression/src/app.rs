use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

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
    // State for file compression
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
                                    is_processing.set(true);
                                    // TODO: Call our Rust compression backend
                                    compression_result.set(Some("Compression simulated - 58% reduction!".to_string()));
                                    is_processing.set(false);
                                }
                            >
                                {move || if is_processing.get() { "PROCESSING..." } else { "COMPRESS" }}
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
