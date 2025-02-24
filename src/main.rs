mod components;
mod parser;
mod render;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    render::render();
}
