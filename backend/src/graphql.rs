use serde::{Deserialize, Serialize};

const QUERY: &str = include_str!("query.graphql");

#[derive(Debug, Serialize)]
pub struct Variables {
    pub login: String,
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Request {
    pub query: String,
    pub variables: Variables,
}

impl Request {
    pub fn new(login: String, cursor: Option<String>) -> Self {
        Self {
            query: QUERY.to_string(),
            variables: Variables { login, cursor },
        }
    }
}

#[derive(Deserialize)]
pub struct Response {
    pub data: Data,
}

#[derive(Deserialize)]
pub struct Data {
    pub user: User,
}

#[derive(Deserialize)]
pub struct User {
    pub name: Option<String>,
    pub repositories: Repositories,
}

#[derive(Deserialize)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,

    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct Repositories {
    pub edges: Vec<RepositoryEdge>,

    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Deserialize)]
pub struct RepositoryEdge {
    pub node: RepositoryNode,
}

#[derive(Deserialize)]
pub struct RepositoryNode {
    #[serde(rename = "nameWithOwner")]
    pub name: String,
    #[serde(rename = "stargazerCount")]
    pub stargazer_count: u32,
    pub languages: Languages,
}

#[derive(Deserialize)]
pub struct Languages {
    pub edges: Vec<LanguagesEdge>,
}

#[derive(Deserialize)]
pub struct LanguagesEdge {
    pub size: u32,
    pub node: LanguageNode,
}

#[derive(Deserialize)]
pub struct LanguageNode {
    pub name: String,
    pub color: Option<String>,
}
