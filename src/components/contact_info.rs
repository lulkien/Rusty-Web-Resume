use serde::{Deserialize, Serialize};
use yew::{html, Html};
use yew_icons::{Icon, IconId};

use super::AsHtml;

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct ContactInfo {
    pub fullname: String,
    pub avatar: Option<String>,
    pub title: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
}

#[allow(unused)]
#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct CustomField {
    pub name: String,
    pub data: String,
}

impl AsHtml for ContactInfo {
    fn as_html(&self) -> yew::Html {
        html! {
            <div class="contact">
                <div class="contact-layout">
                    { self.render_contact_info() }
                    { self.render_avatar() }
                </div>
                <div class="separate-line"></div>
            </div>
        }
    }
}

impl ContactInfo {
    fn render_avatar(&self) -> Html {
        if let Some(avatar) = &self.avatar {
            let avatar_path = format!("assets/{avatar}");
            html! {
                <div class="contact-layout-right">
                    <img src={avatar_path} alt="avatar image" width=144 height=187/>
                </div>
            }
        } else {
            html!()
        }
    }

    fn render_contact_info(&self) -> Html {
        html! {
            <div class="contact-layout-left">
                <h1 class="section-title contact-fullname">{ self.fullname.as_str() }</h1>
                { self.render_title() }
                <div class="contact-mandatory">
                    { self.render_location() }
                    { self.render_email() }
                    { self.render_phone() }
                </div>
                <div class="contact-optional">
                    { self.render_github() }
                    { self.render_linkedin() }
                    { self.render_website() }
                </div>
            </div>
        }
    }

    fn render_title(&self) -> Html {
        self.title.as_ref().map_or(html!(), |title| html!(<p class="contact-job-title">{ title.as_str() }</p>))
    }

    fn render_location(&self) -> Html {
        self.location.as_ref().map_or(html!(), |location| html! {
            <p class="contact-item">
                { Self::get_info_icon(IconId::OcticonsLocation16) }
                { location.as_str() }
            </p>
        })
    }

    fn render_email(&self) -> Html {
        self.email.as_ref().map_or(html!(), |email| html! {
            <p class="contact-item">
                { Self::get_info_icon(IconId::BootstrapEnvelope) }
                <a class="clickable-link" href={ format!("mailto:{}", email.as_str()) }>
                    { email.as_str() }
                </a>
            </p>
        })
    }

    fn render_phone(&self) -> Html {
        self.phone.as_ref().map_or(html!(), |phone| html! {
            <p class="contact-item">
                { Self::get_info_icon(IconId::BootstrapTelephone) }
                <a class="clickable-link" href={ format!("tel:{}", phone.as_str()) }>
                    { phone.as_str() }
                </a>
            </p>
        })
    }

    fn render_website(&self) -> Html {
        self.website.as_ref().map_or(html!(), |website| html! {
            <p class="contact-item">
                { Self::get_info_icon(IconId::BootstrapGlobe2) }
                <a class="clickable-link" href={ Self::get_website_url(website) }>
                    { website.as_str() }
                </a>
            </p>
        })
    }

    fn render_linkedin(&self) -> Html {
        self.linkedin.as_ref().map_or(html!(), |linkedin| html! {
            <p class="contact-item">
                { Self::get_info_icon(IconId::BootstrapLinkedin) }
                <a class="clickable-link" href={ Self::get_linkedin_url(linkedin) }>
                    { linkedin.as_str() }
                </a>
            </p>
        })
    }

    fn render_github(&self) -> Html {
        self.github.as_ref().map_or(html!(), |github| html! {
            <p class="contact-item">
                { Self::get_info_icon(IconId::BootstrapGithub) }
                <a class="clickable-link" href={ Self::get_github_url(github) }>
                    { github.as_str() }
                </a>
            </p>
        })
    }

    fn get_website_url(website: &str) -> String {
        if website.starts_with("https://")
            || website.starts_with("http://")
        {
            website.to_string()
        } else {
            format!("https://{}", website)
        }
    }

    fn get_linkedin_url(linkedin: &str) -> String {
        format!("https://linkedin.com/in/{}", linkedin)
    }

    fn get_github_url(github: &str) -> String {
        format!("https://github.com/{}", github)
    }

    fn get_info_icon(ico: IconId) -> Html {
        html! {
            <Icon class="icon" icon_id={ico} width={"1.3em".to_owned()} height={"1.1em".to_owned()}/>
        }
    }
}
