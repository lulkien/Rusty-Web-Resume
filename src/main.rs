mod components;
mod parser;
mod render;

use parser::parse_resume_data;
use yew::prelude::*;

#[function_component(CV)]
fn cv() -> Html {
    html! {
        <> { "Hello" } </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let _ = parse_resume_data();

    yew::Renderer::<CV>::new().render();
}
