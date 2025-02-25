use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ExternalLinkProps {
    pub text: String,

    #[prop_or(None)]
    pub url: Option<String>,

    #[prop_or(None)]
    pub class: Option<String>,
}

#[function_component(ExternalLink)]
pub fn external_link(props: &ExternalLinkProps) -> Html {
    let classes = format!(
        "external-link{}",
        props
            .class
            .as_ref()
            .map(|c| format!(" {}", c))
            .unwrap_or_default()
    );

    html! {
        <a class={ classes } href={ props.url.clone().unwrap_or(props.text.clone()) }>{ &props.text }</a>
    }
}
