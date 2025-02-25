use serde::{Deserialize, Serialize};
use yew::{html, Html};

use super::AsHtml;
use crate::components::{ExternalLink, SectionTitle};

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
    fn as_html(&self) -> Html {
        let education_items: Vec<Html> = self
            .0
            .iter()
            .map(|item| item.as_html())
            .collect();

        html! {
            <div class="education">
                <SectionTitle title="Educations"/>
                { education_items }
            </div>
        }
    }
}

impl AsHtml for EducationItem {
    fn as_html(&self) -> Html {
        html! {
            <div class="education-item">
                <div class="education-info">
                    <div class="education-info-left">
                        <p class="education-institution highlight-text">{ self.institution.as_str() }</p>
                        { self
                            .area_of_study
                            .as_ref()
                            .map_or_else(|| html!(), |area| html!(
                                <p class="education-area-of-study">{ area.as_str() }</p>
                            )) }
                    </div>
                    <div class="education-info-right">
                        <p class="education-date-range highlight-text">{ self.date_range.as_str() }</p>
                        { self
                            .degree
                            .as_ref()
                            .map_or_else(|| html!(), |degree| html!(
                                <p class="education-degree">{ degree.as_str() }</p>
                            )) }
                    </div>
                </div>
                { self.website.as_ref().map_or_else(|| html!(), |url| html!(
                    <p class="education-website">
                        <strong>{ "Website: " }</strong>
                        <ExternalLink text={ url.to_owned() } />
                    </p>
                ))}
                { self.summary.as_ref().map_or_else(|| html!(), |summary| html!(
                    <p class="education-summary">
                        { summary.as_str() }
                    </p>
                ))}
            </div>
        }
    }
}
