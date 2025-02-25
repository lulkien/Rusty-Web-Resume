use serde::{Deserialize, Serialize};
use yew::{html, Html};
use yew_icons::IconId;

use super::AsHtml;
use crate::components::{
    HeaderTitle, IconLink, IconText, NormalText,
};

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
                    { self.render_avatar() }
                    { self.render_contact_info() }
                </div>
                <div class="separate-line"></div>
            </div>
        }
    }
}

impl ContactInfo {
    fn render_avatar(&self) -> Html {
        if let Some(_avatar) = &self.avatar {
            html! {
                <div style="padding: 20px;">
                    { "Left Section" }
                </div>
            }
        } else {
            html!()
        }
    }

    fn render_contact_info(&self) -> Html {
        html! {
            <div class="contact-layout-right">
                <HeaderTitle title={ self.fullname.to_owned() } />
                <NormalText class="contact-job-title" text={ self.title.clone() } />
                <div class="contact-mandatory">
                    <IconText icon_id={ IconId::OcticonsLocation16 } text={ self.location.clone() } />
                    <IconLink
                        icon_id={ IconId::BootstrapEnvelope }
                        text={ self.email.clone() }
                        url={ format!("mailto:{}", self.email.clone().unwrap_or_default() ) } />
                    <IconLink
                        icon_id={ IconId::BootstrapTelephone }
                        text={ self.phone.clone() }
                        url={ format!("tel:{}", self.phone.clone().unwrap_or_default() ) } />
                </div>
                <div class="contact-optional">
                    <IconLink
                        icon_id={ IconId::BootstrapGithub }
                        text={ self.github.clone() }
                        url={ Self::get_github_url(&self.github.clone().unwrap_or_default()) } />
                    <IconLink
                        icon_id={ IconId::BootstrapLinkedin }
                        text={ self.linkedin.clone() }
                        url={ Self::get_linkedin_url(&self.linkedin.clone().unwrap_or_default()) } />
                    <IconLink
                        icon_id={ IconId::BootstrapLink45Deg }
                        text={ self.website.clone() }
                        url={ Self::get_website_url(&self.website.clone().unwrap_or_default()) } />
                </div>
            </div>
        }
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
}
