use serde::{Deserialize, Serialize};
use yew::{html, Html};

use super::AsHtml;

#[derive(Serialize, Deserialize)]
pub struct Experience(pub Vec<ExperienceItem>);

#[derive(Serialize, Deserialize)]
pub struct ExperienceItem {
    pub company: String,
    pub position: String,
    pub date_range: String,
}

impl AsHtml for Experience {
    fn as_html(&self) -> Html {
        let experience_items: Vec<Html> = self
            .0
            .iter()
            .map(|item| item.as_html())
            .collect();

        html! {
            <div class="experience">
                <h2 class="section-title">{ "Experience" }</h2>
                { experience_items }
            </div>
        }
    }
}

impl AsHtml for ExperienceItem {
    fn as_html(&self) -> Html {
        html! {
            <div class="experience-item">
                <div class="experience-info">
                    <div class="experience-info-left">
                        <p class="experience-company highlight-text">{ self.company.as_str() }</p>
                        <p class="experience-position">{ self.position.as_str() }</p>
                    </div>
                    <div class="experience-info-right">
                        <p class="experience-date-range highlight-text">{ self.date_range.as_str() }</p>
                    </div>
                </div>
            </div>
        }
    }
}
