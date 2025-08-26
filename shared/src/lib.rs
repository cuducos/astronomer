use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Partial {
    pub repository: String,
    pub stars: f64,
}

#[derive(Clone, Serialize)]
pub struct Language {
    pub name: String,
    pub stars: f64,
    pub color: String,
    pub source: Vec<Partial>,

    #[serde(skip)]
    pub lines: u32,
}

impl Language {
    pub fn new(name: String, color: String) -> Self {
        Self {
            name,
            stars: 0.0,
            color,
            source: vec![],
            lines: 0,
        }
    }

    pub fn merge(&self, old: &Self) -> Self {
        let mut source = self.source.clone();
        source.extend(old.source.clone());
        Self {
            name: self.name.clone(),
            stars: self.stars + old.stars,
            lines: self.lines + old.lines,
            color: old.color.clone(),
            source,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Repository {
    pub name: String,
    pub languages: Vec<Language>,
    pub stars: u32,
}
