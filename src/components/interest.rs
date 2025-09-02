use serde::{Deserialize, Serialize};
use yew::{html, Html};

use super::AsHtml;

#[derive(Serialize, Deserialize)]
pub struct Interests(pub Vec<String>);

impl AsHtml for Interests {
    fn as_html(&self) -> Html {
        let interest: Vec<Html> = self
            .0
            .iter()
            .map(|item| {
                html! {
                    <p class="interest-item">{ item }</p>
                }
            })
            .collect();

        html! {
            <div class="interest">
                <h2 class="section-title">{ "Interests" }</h2>
                { interest }
            </div>
        }
    }
}
