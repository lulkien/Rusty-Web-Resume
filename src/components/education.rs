use serde::{Deserialize, Serialize};
use yew::{html, Html};

use super::AsHtml;

#[derive(Serialize, Deserialize)]
pub struct Education(pub Vec<EducationItem>);

#[derive(Serialize, Deserialize)]
pub struct EducationItem {
    pub institution: String,
    pub degree: Option<String>,
    pub area_of_study: Option<String>,
    pub date_range: String,
    pub score: Option<f32>,
    pub website: Option<String>,
    pub summary: Option<String>,
}

impl AsHtml for Education {
    fn as_html(&self) -> yew::Html {
        let education_items: Vec<Html> = self.0
            .iter()
            .map(|item| {
                html! {
                    <div class="education-item">
                        <h3>{ &item.institution }</h3>
                        { item.degree.as_ref().map(|degree| html! { <p><strong>{ "Degree: " }</strong>{ degree }</p> }) }
                        { item.area_of_study.as_ref().map(|area| html! { <p><strong>{ "Area of Study: " }</strong>{ area }</p> }) }
                        <p><strong>{ "Date Range: " }</strong>{ &item.date_range }</p>
                        { item.score.map(|score| html! { <p><strong>{ "Score: " }</strong>{ score }</p> }) }
                        { item.website.as_ref().map(|website| html! { <p><strong>{ "Website: " }</strong><a href={ website.clone() }>{ website }</a></p> }) }
                        { item.summary.as_ref().map(|summary| html! { <p><strong>{ "Summary: " }</strong>{ summary }</p> }) }
                    </div>
                }
            })
            .collect();

        html! {
            <div class="education-section">
                <h2>{ "Education" }</h2>
                { education_items }
            </div>
        }
    }
}
