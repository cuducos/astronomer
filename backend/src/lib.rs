use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};
use shared::{Language, Partial, Repository};
use std::iter::FromIterator;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
};

mod graphql;

const API_URL: &str = "https://api.github.com/graphql";
const DEFAULT_COLOR: &str = "#efefef";

#[derive(Debug)]
pub enum Error {
    Serializer(serde_json::Error),
    Http(reqwest::Error),
    HttpStatus(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::Serializer(err) => write!(f, "Serializer error: {err}"),
            Error::Http(err) => write!(f, "HTTP error: {err}"),
            Error::HttpStatus(status) => write!(f, "HTTP status error: {status}"),
        }
    }
}

#[derive(Clone, Serialize)]
struct User {
    name: String,
    languages: Vec<Language>,
    stars: u32,

    login: String,

    #[serde(skip)]
    repositories: Vec<Repository>,
}

impl User {
    fn new(login: String) -> Self {
        Self {
            login,
            languages: Vec::new(),
            stars: 0,
            repositories: Vec::new(),
            name: "".to_string(),
        }
    }

    fn store_repos(&mut self, repos: Vec<graphql::RepositoryEdge>) {
        for edge in &repos {
            let languages = edge
                .node
                .languages
                .edges
                .iter()
                .map(|edge| Language {
                    name: edge.node.name.clone(),
                    lines: edge.size,
                    color: edge
                        .node
                        .color
                        .as_deref()
                        .unwrap_or(DEFAULT_COLOR)
                        .to_string(),
                    stars: 0.0,
                    source: vec![],
                })
                .collect();

            self.repositories.push(Repository {
                name: edge.node.name.clone(),
                stars: edge.node.stargazer_count,
                languages,
            });
        }
    }

    fn calculate(&mut self) {
        self.stars = self.repositories.iter().map(|repo| repo.stars).sum();
        for i in 0..self.repositories.len() {
            let repo = &mut self.repositories[i];
            let total_lines = repo.languages.iter().map(|lang| lang.lines).sum::<u32>();
            for idx in 0..repo.languages.len() {
                let language = &mut repo.languages[idx];
                let language_weight = language.lines as f64 / total_lines as f64;
                language.stars = repo.stars as f64 * language_weight;
            }
        }
    }

    fn merge_and_sort(&mut self) {
        let mut non_sorted = HashMap::<String, Language>::new();
        for repo in self.repositories.iter_mut() {
            for language in repo.languages.iter_mut() {
                let current = match non_sorted.get(&language.name) {
                    Some(current) => current.clone(),
                    None => Language::new(language.name.clone(), language.color.clone()),
                };
                if language.stars > 0.0 {
                    language.source.push(Partial {
                        repository: repo.name.clone(),
                        stars: language.stars,
                    });
                }
                non_sorted.insert(language.name.clone(), language.merge(&current));
            }
        }
        self.languages = Vec::from_iter(non_sorted.into_values().filter(|lang| lang.stars > 0.0));
        self.languages
            .sort_by(|a, b| b.stars.partial_cmp(&a.stars).unwrap());
        for language in self.languages.iter_mut() {
            language
                .source
                .sort_by(|a, b| b.stars.partial_cmp(&a.stars).unwrap());
        }
    }

    async fn load(&mut self, is_archived: Option<bool>, token: String) -> Result<(), Error> {
        let mut cursor: Option<String> = None;
        loop {
            let query = graphql::Request::new(self.login.to_string(), is_archived, cursor);
            let body = serde_json::to_string(&query).map_err(Error::Serializer)?;
            let resp = reqwest::Client::new()
                .post(API_URL)
                .header("User-Agent", &self.login)
                .bearer_auth(&token)
                .body(body)
                .send()
                .await
                .map_err(Error::Http)?;
            if !resp.status().is_success() {
                eprintln!("{}", &resp.status().as_str());
                return Err(Error::HttpStatus(resp.text().await.map_err(Error::Http)?));
            }
            let body = resp
                .json::<graphql::Response>()
                .await
                .map_err(Error::Http)?;
            self.store_repos(body.data.user.repositories.edges);
            self.name = body.data.user.name.unwrap_or_else(|| self.login.clone());
            if !body.data.user.repositories.page_info.has_next_page {
                break;
            }
            cursor = body.data.user.repositories.page_info.end_cursor;
        }
        self.calculate();
        self.merge_and_sort();
        Ok(())
    }
}

#[cached(time = 86400, result = true)] // one day cache
async fn cached_json_for(
    login: String,
    is_archived: Option<bool>,
    token: String,
) -> Result<User, Error> {
    let mut user = User::new(login);
    user.load(is_archived, token).await?;
    Ok(user)
}

#[derive(Deserialize)]
pub struct RawConfig {
    exclude: Option<String>,
    top: Option<usize>,
    status: Option<String>,
}

pub enum Status {
    All,
    Active,
    Archived,
}

pub struct Config {
    exclude: HashSet<String>,
    top: usize,
    status: Status,
}

impl Config {
    pub fn from_raw(config: &RawConfig) -> Self {
        let status = config
            .status
            .as_ref()
            .map(|value| match value.to_lowercase().as_str() {
                "archived" => Status::Archived,
                "active" => Status::Active,
                _ => Status::All,
            })
            .unwrap_or(Status::All);
        Self {
            exclude: config
                .exclude
                .as_ref()
                .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            top: config.top.unwrap_or(0),
            status,
        }
    }
}

pub async fn json_for(login: String, token: String, config: Config) -> Result<String, Error> {
    let is_archived = match config.status {
        Status::All => None,
        Status::Active => Some(false),
        Status::Archived => Some(true),
    };
    let mut user = cached_json_for(login, is_archived, token).await?;
    if !config.exclude.is_empty() {
        user.languages.retain(|l| !config.exclude.contains(&l.name));
    }
    if config.top > 0 {
        user.languages = user.languages.into_iter().take(config.top).collect();
    }
    serde_json::to_string(&user).map_err(Error::Serializer)
}
