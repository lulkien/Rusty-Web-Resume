use serde::{Deserialize, Serialize};
use yew::{html, Html};

use super::AsHtml;

#[derive(Serialize, Deserialize)]
pub struct Languages(pub Vec<LanguageItem>);

#[derive(Serialize, Deserialize)]
pub struct LanguageItem {
    pub name: String,
    pub description: Option<String>,
}

impl AsHtml for Languages {
    fn as_html(&self) -> Html {
        let languages: Vec<Html> = self
            .0
            .iter()
            .map(|item| item.as_html())
            .collect();

        html! {
            <div class="language">
                <h2 class="section-title">{ "Languages" }</h2>
                { languages }
            </div>
        }
    }
}

impl AsHtml for LanguageItem {
    fn as_html(&self) -> Html {
        html! {
            <div class="language-item">
                <p class="language-name highlight-text">{ self.name.as_str() }</p>
                { self.description.as_ref().map_or_else(|| html!(), |description| html!(
                    <p class="language-description">{ description }</p>
                ))}
            </div>
        }
    }
}
