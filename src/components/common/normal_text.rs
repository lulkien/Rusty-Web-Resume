use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct NormalTextProps {
    #[prop_or(None)]
    pub text: Option<String>,

    #[prop_or(None)]
    pub class: Option<String>,
}

#[function_component(NormalText)]
pub fn normal_text(props: &NormalTextProps) -> Html {
    if props.text.is_none() {
        return html!();
    }

    let classes =
        props.class.clone().unwrap_or("".to_owned());

    html! {
        <p class={ classes }> { &props.text } </p>
    }
}
