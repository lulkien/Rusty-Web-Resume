use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SectionTitleProps {
    pub title: String,
}

#[function_component(SectionTitle)]
pub fn section_title(props: &SectionTitleProps) -> Html {
    html! {
        <h2 class="section-title">{ &props.title }</h2>
    }
}
