use actix_files::Files;
use actix_web::{
    get, http::header::ContentType, web, App, Error, HttpResponse, HttpServer, ResponseError,
};
use derive_more::Display;
use log::{error, info};

const TEMPLATE: &str = include_str!("index.html");
const DEFAULT_USER_PATH: &str = "/cuducos";
const TOKEN: &str = "ASTRONOMER_GITHUB_TOKEN";

#[derive(Debug, Display)]
enum HttpError {
    #[display("Missing token on the server side.")]
    MissingToken,

    #[display("Could not connect to GitHUb or reading the response.")]
    GitHubClientError,
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HttpError::MissingToken => {
                error!("Missing {} environment variable.", TOKEN);
                HttpResponse::InternalServerError().finish()
            }
            HttpError::GitHubClientError => {
                error!("Error connecting to GitHub or reading their response.");
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[get("/{name}.json")]
async fn api(
    name: web::Path<String>,
    config: web::Query<backend::RawConfig>,
) -> Result<HttpResponse, Error> {
    let output = backend::json_for(
        name.to_string(),
        std::env::var(TOKEN).map_err(|_| HttpError::MissingToken)?,
        backend::Config::from_raw(&config),
    )
    .await
    .map_err(|_| HttpError::GitHubClientError)?;
    Ok(HttpResponse::Ok()
        .insert_header(ContentType(mime::APPLICATION_JSON))
        .body(output))
}

#[get("/{name}")]
async fn user(name: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_HTML_UTF_8))
        .body(
            TEMPLATE
                .replace("${USERNAME}", name.as_str())
                .replace('\n', "")
                .replace("   ", ""),
        ))
}

#[get("/{name}/")]
async fn redirect(name: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", format!("/{name}")))
        .finish())
}

#[get("/")]
async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::TemporaryRedirect()
        .append_header(("Location", DEFAULT_USER_PATH))
        .finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    if std::env::var(TOKEN).is_err() {
        error!("Missing {} environment variable.", TOKEN);
        std::process::exit(1);
    }
    let port = std::env::var("PORT")
        .and_then(|port| {
            port.parse::<u16>()
                .map_err(|_| std::env::VarError::NotPresent)
        })
        .unwrap_or(8000);
    info!("Starting server at http://0.0.0.0:{}", port);
    HttpServer::new(move || {
        App::new()
            .service(api)
            .service(user)
            .service(redirect)
            .service(home)
            .service(Files::new("/static", "./static"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
