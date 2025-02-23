use yew::{html, Html};

const STYLE_STR: &str =
    include_str!("../config/styles/default.css");

pub fn get_page_style() -> Html {
    html!(
        <style> { STYLE_STR } </style>
    )
}
