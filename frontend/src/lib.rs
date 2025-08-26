use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use shared::Language;
use std::num::ParseIntError;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

fn parse_rgb(hex: &str) -> Result<(u8, u8, u8), ParseIntError> {
    Ok((
        u8::from_str_radix(&hex[1..3], 16)?,
        u8::from_str_radix(&hex[3..5], 16)?,
        u8::from_str_radix(&hex[5..7], 16)?,
    ))
}

fn color(language: &Language, position: usize) -> String {
    if language.source.len() <= 1 || !language.color.starts_with('#') || language.color.len() != 7 {
        return language.color.clone();
    }
    parse_rgb(language.color.as_str())
        .map(|(r, g, b)| {
            let p = 1.0 - (position as f32) / ((language.source.len() - 1) as f32);
            let a = 0.25 + p * 0.75;
            format!("rgba({r}, {g}, {b}, {a})")
        })
        .unwrap_or(language.color.clone())
}

#[derive(Serialize, Deserialize)]
struct Dataset {
    label: String,
    data: Vec<f64>,
    language: String,
    total: f64,

    #[allow(non_snake_case)]
    backgroundColor: String,

    #[allow(non_snake_case)]
    borderRadius: u32,
}

#[derive(Serialize, Deserialize)]
struct Data {
    labels: Vec<String>,
    datasets: Vec<Dataset>,
}

fn _data_for(languages: Vec<Language>) -> Data {
    let total = languages.len();
    let mut labels: Vec<String> = Vec::with_capacity(total);
    let mut datasets: Vec<Dataset> = Vec::with_capacity(total);
    for (idx, language) in languages.iter().enumerate() {
        labels.push(language.name.clone());
        for (i, source) in language.source.iter().enumerate() {
            let mut data: Vec<f64> = vec![0.0; total];
            data[idx] = source.stars;
            datasets.push(Dataset {
                label: source.repository.clone(),
                data,
                backgroundColor: color(language, i),
                borderRadius: 2,
                language: language.name.clone(),
                total: language.stars,
            });
        }
    }
    Data { labels, datasets }
}

#[wasm_bindgen]
pub fn data_for(raw: Vec<JsValue>) -> JsValue {
    let mut languages: Vec<Language> = Vec::with_capacity(raw.len());
    for value in raw.into_iter() {
        match from_value::<Language>(value) {
            Ok(language) => languages.push(language),
            Err(e) => {
                console::error_1(&e.into());
                return JsValue::NULL;
            }
        };
    }
    let dataset = _data_for(languages);
    match to_value(&dataset) {
        Ok(val) => val,
        Err(e) => {
            console::error_1(&e.into());
            JsValue::NULL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::Partial;

    #[test]
    fn test_color_single_color() {
        let language = Language {
            name: "FooBar".to_string(),
            color: "#ff5733".to_string(),
            stars: 1.0,
            lines: 1,
            source: vec![Partial {
                repository: "foo/bar".to_string(),
                stars: 1.0,
            }],
        };
        assert_eq!(color(&language, 1), "#ff5733");
        assert_eq!(color(&language, 0), "#ff5733");
    }

    #[test]
    fn test_color_multiple_colors() {
        let language = Language {
            name: "FooBar".to_string(),
            color: "#ff5733".to_string(),
            stars: 3.0,
            lines: 3,
            source: vec![
                Partial {
                    repository: "foo/bar".to_string(),
                    stars: 1.0,
                },
                Partial {
                    repository: "foo/baz".to_string(),
                    stars: 1.0,
                },
                Partial {
                    repository: "foo/barbaz".to_string(),
                    stars: 1.0,
                },
            ],
        };
        assert_eq!(color(&language, 0), "rgba(255, 87, 51, 1)");
        assert_eq!(color(&language, 1), "rgba(255, 87, 51, 0.625)");
        assert_eq!(color(&language, 2), "rgba(255, 87, 51, 0.25)");
        // assert_eq!(&language, 0, 10), "rgba(255, 87, 51, 1)");
        // assert_eq!(
        //     &language, 9, 10),
        //     "rgba(255, 87, 51, 0.25)"
        // );
    }

    #[test]
    fn test_color_with_invalid_hex() {
        let language = Language {
            name: "FooBar".to_string(),
            color: "#zzzzzz".to_string(),
            stars: 1.0,
            lines: 1,
            source: vec![Partial {
                repository: "foo/bar".to_string(),
                stars: 1.0,
            }],
        };
        assert_eq!(color(&language, 0), "#zzzzzz");
    }

    #[test]
    fn test_data_for() {
        let languages = vec![
            Language {
                name: "Foo".to_string(),
                color: "#ff5733".to_string(),
                stars: 3.0,
                lines: 42,
                source: vec![
                    Partial {
                        repository: "bar/foo".to_string(),
                        stars: 1.0,
                    },
                    Partial {
                        repository: "baz/foo".to_string(),
                        stars: 2.0,
                    },
                ],
            },
            Language {
                name: "Bar".to_string(),
                color: "#007acc".to_string(),
                stars: 0.1,
                lines: 4,
                source: vec![Partial {
                    repository: "foo/bar".to_string(),
                    stars: 0.1,
                }],
            },
        ];
        let data = _data_for(languages);
        assert_eq!(data.labels.len(), 2);
        assert_eq!(data.labels[0], "Foo".to_string());
        assert_eq!(data.labels[1], "Bar".to_string());

        assert_eq!(data.datasets.len(), 3);
        assert_eq!(data.datasets[0].label, "bar/foo");
        assert_eq!(data.datasets[0].language, "Foo");
        assert_eq!(data.datasets[0].total, 3.0);
        assert_eq!(data.datasets[0].data, [1.0, 0.0]);

        assert_eq!(data.datasets[1].label, "baz/foo");
        assert_eq!(data.datasets[1].language, "Foo");
        assert_eq!(data.datasets[1].total, 3.0);
        assert_eq!(data.datasets[1].data, [2.0, 0.0]);

        assert_eq!(data.datasets[2].label, "foo/bar");
        assert_eq!(data.datasets[2].language, "Bar");
        assert_eq!(data.datasets[2].total, 0.1);
        assert_eq!(data.datasets[2].data, [0.0, 0.1]);
    }
}
