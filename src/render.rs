use crate::{
    components::{AsHtml, Resume},
    parser, style,
};

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let resume: Resume =
        parser::parse_resume_data().unwrap();

    let sections = vec![
        Some(resume.contact_info.as_html()),
        resume.summary.as_ref().map(|s| s.as_html()),
        resume.education.as_ref().map(|e| e.as_html()),
        resume.skills.as_ref().map(|s| s.as_html()),
        resume.projects.as_ref().map(|p| p.as_html()),
        // resume.languages.as_ref().map(|l| l.as_html()),
        // resume.interests.as_ref().map(|i| i.as_html()), // Fixed typo: `interests`
    ];

    let page: Vec<Html> =
        sections.into_iter().flatten().collect();

    html! {
        <>
            <title>{ "My CV" }</title>
            { style::get_page_style() }
            <div class="cv-container">
                { page }
            </div>
        </>
    }
}

pub fn render() {
    yew::Renderer::<App>::new().render();
}
