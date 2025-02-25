use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ExternalLinkProps {
    #[prop_or(None)]
    pub url: Option<String>,

    pub text: String,
}

#[function_component(ExternalLink)]
pub fn external_link(props: &ExternalLinkProps) -> Html {
    html! {
        <a class="external-link" href={ props.url.clone().unwrap_or(props.text.clone()) }>{ &props.text }</a>
    }
}
