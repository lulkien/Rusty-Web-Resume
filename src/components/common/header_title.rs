use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct HeaderTitleProps {
    pub title: String,

    #[prop_or(None)]
    pub class: Option<String>,
}

#[function_component(HeaderTitle)]
pub fn header_title(props: &HeaderTitleProps) -> Html {
    let classes = format!(
        "header-title{}",
        props
            .class
            .as_ref()
            .map(|c| format!(" {}", c))
            .unwrap_or_default()
    );

    html! {
        <h1 class={ classes }>{ &props.title }</h1>
    }
}
