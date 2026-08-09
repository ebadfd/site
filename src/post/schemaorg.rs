use serde::{Deserialize, Serialize};

/// schema.org `Article`, emitted as JSON-LD on every post page.
///
/// Google's article rich-result guidance asks for `author`, `publisher`,
/// `dateModified` and `mainEntityOfPage` in addition to the headline/date pair
/// this used to carry; without them the markup parses but is not eligible for
/// article rich results.
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "mainEntityOfPage")]
    pub main_entity_of_page: MainEntity,
    pub headline: String,
    pub description: String,
    pub image: String,
    pub url: String,
    pub author: Person,
    pub publisher: Person,
    #[serde(rename = "datePublished")]
    pub date_published: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct MainEntity {
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "@type")]
    pub r#type: String,
    pub name: String,
    #[serde(rename = "sameAs", skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
}

/// schema.org `SoftwareApplication`, for pages under `/products`.
///
/// No `offers` or `aggregateRating`: Google wants both for the software
/// rich result, but neither can be filled in honestly from what a post's
/// frontmatter carries, and inventing a rating is exactly the kind of thing
/// that gets structured data ignored.
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareApplication {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub image: String,
    #[serde(rename = "applicationCategory")]
    pub application_category: String,
    pub author: Person,
    #[serde(rename = "datePublished")]
    pub date_published: String,
}

/// schema.org `BreadcrumbList` — Home → section → page.
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Breadcrumbs {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "itemListElement")]
    pub items: Vec<Crumb>,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Crumb {
    #[serde(rename = "@type")]
    pub r#type: String,
    pub position: u8,
    pub name: String,
    pub item: String,
}
