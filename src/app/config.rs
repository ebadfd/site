use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "defaultAuthor")]
    pub default_author: Author,
    pub port: u16,
    #[serde(rename = "notableProjects")]
    pub notable_projects: Vec<Link>,
    #[serde(rename = "contactLinks")]
    pub contact_links: Vec<Link>,
    #[serde(rename = "siteTitle")]
    pub site_title: String,
}

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct Link {
    pub url: String,
    pub title: String,
    pub description: String,
}

impl Render for Link {
    fn render(&self) -> Markup {
        html! {
            span {
                a href=(self.url) {(self.title)}
                @if !self.description.is_empty() {
                    ": "
                    (self.description)
                }
            }
        }
    }
}

fn schema_context() -> String {
    "http://schema.org/".to_string()
}

fn schema_person_type() -> String {
    "Person".to_string()
}

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct Author {
    #[serde(rename = "@context", default = "schema_context")]
    pub context: String,
    #[serde(rename = "@type", default = "schema_person_type")]
    pub schema_type: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub handle: String,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub pic_url: Option<String>,
    #[serde(rename = "inSystem", skip_serializing)]
    pub in_system: bool,
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    #[serde(rename = "sameAs")]
    pub same_as: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
