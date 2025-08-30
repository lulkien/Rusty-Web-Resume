use serde::{Deserialize, Serialize};
use yew::{html, Html};

use super::AsHtml;

#[derive(Serialize, Deserialize)]
pub struct Projects(pub Vec<ProjectItem>);

#[derive(Serialize, Deserialize)]
pub struct ProjectItem {
    pub name: String,
    pub description: String,
    pub date_range: String,
    pub source_code: Option<String>,
    pub website: Option<String>,
    pub demo: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub highlights: Option<Vec<String>>,
}

impl AsHtml for Projects {
    fn as_html(&self) -> Html {
        let projects: Vec<_> = self
            .0
            .iter()
            .map(|item| item.as_html())
            .collect();

        html! {
            <div class="project">
                <h2 class="section-title">{ "Projects" }</h2>
                { projects }
            </div>
        }
    }
}

impl AsHtml for ProjectItem {
    fn as_html(&self) -> Html {
        let highlights: Vec<_> = self
            .highlights
            .as_ref()
            .map_or(vec![], |list| {
                list
                    .iter()
                    .filter(|item| !item.is_empty())
                    .map(|item| {
                        html!( <li> { item.as_str() } </li>)
                    })
                    .collect()
            });

        let keywords_str: Html = self
            .keywords
            .as_ref()
            .map_or(html!(), |list_keyword| {
                html! {
                    <p class="project-keywords">
                        <strong>{"Keywords: "}</strong>
                        { list_keyword.join(", ") }
                    </p>
                }
            });

        html! {
            <div class="project-item">
                <div class="project-info">
                    <div class="project-info-left">
                        <p class="project-name highlight-text">{ self.name.as_str() }</p>
                    </div>
                    <div class="project-info-right">
                        <p class="project-date-range highlight-text">{ self.date_range.as_str() }</p>
                    </div>
                </div>
                { keywords_str }
                { self.source_code.as_ref().map_or(html!(), |url| html!(
                    <p class="project-source-code">
                        <strong>{ "Source: " }</strong>
                        <a class="clickable-link" href={ url.clone() }>{ url.as_str() }</a>
                    </p>
                ))}
                { self.website.as_ref().map_or(html!(), |url| html!(
                    <p class="project-website">
                        <strong>{ "Website: " }</strong>
                        <a class="clickable-link" href={ url.clone() }>{ url.as_str() }</a>
                    </p>
                ))}
                { self.demo.as_ref().map_or(html!(), |url| html!(
                    <p class="project-demo">
                        <strong>{ "Demo: " }</strong>
                        <a class="clickable-link" href={ url.clone() }>{ url.as_str() }</a>
                    </p>
                ))}
                <p class="project-description">{ self.description.as_str() }</p>
                <ul>
                    { for highlights }
                </ul>
            </div>
        }
    }
}
