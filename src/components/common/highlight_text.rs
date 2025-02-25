use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct HighlightTextProps {
    pub text: String,

    #[prop_or(None)]
    pub class: Option<String>,
}

#[function_component(HighlightText)]
pub fn highlight_text(props: &HighlightTextProps) -> Html {
    let classes = format!(
        "highlight-text{}",
        props
            .class
            .as_ref()
            .map(|c| format!(" {}", c))
            .unwrap_or_default()
    );

    html! {
        <p class={ classes }>
            { &props.text }
        </p>
    }
}
