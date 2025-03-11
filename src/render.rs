use crate::{
    components::{AsHtml, Resume},
    parser,
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
        resume.projects.as_ref().map(|p| p.as_html()),
        resume.skills.as_ref().map(|s| s.as_html()),
        resume.languages.as_ref().map(|l| l.as_html()),
        resume.interests.as_ref().map(|i| i.as_html()),
    ];

    let page: Vec<Html> =
        sections.into_iter().flatten().collect();

    html! {
        <>
            <div class="cv-container">
                { page }
            </div>
            <Footer/>
        </>

    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer style="text-align: center; padding: 20px; background-color: #f0f0f0;">
            <p>
                {"Made with ❤️ using "}
                <a href="https://yew.rs" target="_blank" rel="noopener noreferrer">
                    {"Yew"}
                </a>
                {"  |  Source code on "}
                <a href="https://github.com/lulkien/Rusty-Web-CV" target="_blank" rel="noopener noreferrer">
                    {"GitHub"}
                </a>
            </p>
        </footer>
    }
}

pub fn render() {
    yew::Renderer::<App>::new().render();
}
