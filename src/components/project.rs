use serde::{Deserialize, Serialize};
use yew::{html, Html};

use super::AsHtml;

#[derive(Serialize, Deserialize)]
pub struct Projects(pub Vec<ProjectItem>);

#[derive(Serialize, Deserialize)]
pub struct ProjectItem {
    pub name: String,
    pub description: Option<String>,
    pub date_range: String,
    pub website: Option<String>,
    pub summary: Option<String>,
}

impl AsHtml for Projects {
    fn as_html(&self) -> Html {
        html! {
            <div class="projects">
                <h2 class="section-title">{ "Projects" }</h2>
            </div>
        }
    }
}

impl AsHtml for ProjectItem {
    fn as_html(&self) -> Html {
        todo!()
    }
}
