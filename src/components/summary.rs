use serde::{Deserialize, Serialize};
use yew::prelude::*;

use super::AsHtml;

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct Summary(pub String);

impl AsHtml for Summary {
    fn as_html(&self) -> Html {
        html! {
            <div class="summary">
                <h2 class="section-title">{ "Summary" }</h2>
                <p>{ self.0.as_str() }</p>
            </div>
        }
    }
}
