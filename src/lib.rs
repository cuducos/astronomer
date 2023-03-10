use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use cached::proc_macro::cached;
use serde::Serialize;

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

#[derive(Serialize)]
struct Language {
    name: String,
    stars: f64,
    color: String,

    #[serde(skip)]
    lines: u32,
}

impl Language {
    fn new(name: String, color: String) -> Self {
        Self {
            name,
            stars: 0.0,
            lines: 0,
            color,
        }
    }

    fn clone(old: &Self) -> Self {
        Self {
            name: old.name.clone(),
            stars: old.stars,
            lines: old.lines,
            color: old.color.clone(),
        }
    }

    fn merge(&self, old: &Self) -> Self {
        Self {
            name: self.name.clone(),
            stars: self.stars + old.stars,
            lines: self.lines + old.lines,
            color: old.color.clone(),
        }
    }
}

#[derive(Serialize)]
struct Repository {
    languages: Vec<Language>,
    stars: u32,
}

#[derive(Serialize)]
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
                })
                .collect();

            self.repositories.push(Repository {
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
        for repo in &self.repositories {
            for language in &repo.languages {
                let current = match non_sorted.get(&language.name) {
                    Some(current) => Language::clone(current),
                    None => Language::new(language.name.clone(), language.color.clone()),
                };
                non_sorted.insert(language.name.clone(), language.merge(&current));
            }
        }
        self.languages = Vec::from_iter(non_sorted.into_values().filter(|lang| lang.stars > 0.0));
        self.languages
            .sort_by(|a, b| b.stars.partial_cmp(&a.stars).unwrap());
    }

    async fn load(&mut self, token: String) -> Result<(), Error> {
        let mut cursor: Option<String> = None;
        loop {
            let query = graphql::Request::new(self.login.to_string(), cursor);
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

#[cached(time = 604800, result = true)] // one week cache
pub async fn json_for(login: String, token: String) -> Result<String, Error> {
    let mut user = User::new(login);
    user.load(token).await?;
    serde_json::to_string(&user).map_err(Error::Serializer)
}
