use yew::{function_component, html, Html, Properties};
use yew_icons::{Icon, IconId};

use crate::components::ExternalLink;

#[derive(Properties, PartialEq)]
pub struct IconLinkProps {
    #[prop_or(None)]
    pub text: Option<String>,

    pub icon_id: IconId,

    #[prop_or(None)]
    pub url: Option<String>,

    #[prop_or(None)]
    pub class: Option<String>,
}

#[function_component(IconLink)]
pub fn icon_link(props: &IconLinkProps) -> Html {
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
            <ExternalLink
                url={ props.url.clone() }
                text={ props.text.clone().unwrap() } />
        </div>
    }
}
