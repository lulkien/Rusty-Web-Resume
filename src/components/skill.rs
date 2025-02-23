use serde::{Deserialize, Serialize};
use yew::{html, Html};

use super::AsHtml;

#[derive(Serialize, Deserialize)]
pub struct Skills(pub Vec<SkillItem>);

#[derive(Serialize, Deserialize)]
pub struct SkillItem {
    pub name: String,
    pub description: Option<String>,
}

impl AsHtml for Skills {
    fn as_html(&self) -> Html {
        let skillset: Vec<Html> = self
            .0
            .iter()
            .map(|item| item.as_html())
            .collect();

        html! {
        <div class="skills">
            <h2 class="section-title">{ "Skills" }</h2>
            { skillset }
        </div>
        }
    }
}

impl AsHtml for SkillItem {
    fn as_html(&self) -> Html {
        html!(
        <div>
            <p class="skill-name">{ self.name.as_str() }</p>
            { self.description.as_ref().map_or_else(|| html!(), |description| html!(
                <p class="skill-description">{ description }</p>
            ))}
        </div>
        )
    }
}
