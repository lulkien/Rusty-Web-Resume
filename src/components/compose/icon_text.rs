use yew::{function_component, html, Html, Properties};
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct IconTextProps {
    #[prop_or(None)]
    pub text: Option<String>,

    pub icon_id: IconId,

    #[prop_or(None)]
    pub class: Option<String>,
}

#[function_component(IconText)]
pub fn icon_link(props: &IconTextProps) -> Html {
    if props.text.is_none() {
        return html!();
    }

    let classes = format!(
        "icon-info{}",
        props
            .class
            .as_ref()
            .map(|c| format!(" {}", c))
            .unwrap_or_default()
    );

    html! {
        <div class={ classes }>
            <Icon class="icon" icon_id={ props.icon_id } width={"1.3em".to_owned()} height={"1.1em".to_owned() }/>
            <p>{ props.text.clone().unwrap() }</p>
        </div>
    }
}
