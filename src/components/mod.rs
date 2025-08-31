mod contact_info;
mod education;
mod experience;
mod interest;
mod language;
mod project;
mod skill;
mod summary;

use serde::{Deserialize, Serialize};

use contact_info::ContactInfo;
use education::Education;
use experience::Experience;
use interest::Interests;
use language::Languages;
use project::Projects;
use skill::Skills;
use summary::Summary;

pub trait AsHtml {
    fn as_html(&self) -> yew::Html;
}

#[derive(Serialize, Deserialize)]
pub struct Resume {
    pub contact_info: ContactInfo,
    pub summary: Option<Summary>,
    pub experience: Option<Experience>,
    pub education: Option<Education>,
    pub skills: Option<Skills>,
    pub projects: Option<Projects>,
    pub languages: Option<Languages>,
    pub interests: Option<Interests>,
}
