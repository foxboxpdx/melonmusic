//! main.rs
//! MelonMusic front-end binary component
//! This is only really used if the application is compiled as a normal
//! executable, rather than through wasm-pack.
use melonmusic_frontend::app::App;

/// Launch
fn main() {
    yew::Renderer::<App>::new().render();
}
