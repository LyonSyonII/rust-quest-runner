use rocket::serde;
use run::run;
use serde::json::Json;

mod run;
// use crate::run::run;

#[derive(Debug, serde::Deserialize)]
#[serde(crate = "serde", default)]
struct Config {
    #[serde(alias = "auth")]
    authorization: String,
    port: u16,
    semaphore_permits: u8,
    semaphore_wait: u16,
    kill_timeout: u16,
    origins_whitelist: Vec<String>,
    content_length_limit: u16,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            authorization: String::new(),
            port: 3030,
            semaphore_permits: 5,
            semaphore_wait: 500,
            kill_timeout: 500,
            origins_whitelist: Vec::new(),
            content_length_limit: 1024 * 4,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(crate = "serde")]
struct Input {
    code: String,
}

#[derive(serde::Serialize)]
#[serde(crate = "serde")]
struct Output {
    stdout: String,
    stderr: String,
}

#[derive(serde::Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    Std,
    Core,
    ExternC,
    Unsafe,
    TempDir,
    InputFileCreate,
    InputFileOpen,
    InputFileWrite,
    Build,
    Compiler(String),
    Timeout,
    Execution(String),

    NotAuthorized,
    BodyNotCorrect,
}

#[rocket::launch]
async fn launch() -> _ {
    // console_subscriber::init();

    let config: Config = match envy::from_env() {
        Ok(config) => config,
        Err(e) => panic!("{e}"),
    };
    println!("{config:#?}");

    if config.authorization.is_empty() {
        println!(
            "Warning: AUTH environment variable is not set, anyone will be able to send requests!"
        );
    }
    if config.origins_whitelist.is_empty() {
        println!(
            "Warning: ORIGINS_WHITELIST environment variable is not set, anyone will be able to send requests!"
        );
    }

    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge((
            "limits",
            rocket::figment::map!("json" => config.content_length_limit),
        ))
        .merge(rocket::figment::providers::Env::prefixed("APP_").global());
    rocket::custom(figment)
        .mount("/", rocket::routes![default_route, evaluate])
        .manage(tokio::sync::Semaphore::new(
            config.semaphore_permits as usize,
        ))
        .manage(config)
}

#[rocket::get("/evaluate.json")]
fn default_route() -> &'static str {
    "Waiting requests!"
}

#[rocket::post("/evaluate.json", format = "json", data = "<data>")]
async fn evaluate(
    data: Json<Input>,
    semaphore: &rocket::State<tokio::sync::Semaphore>,
    config: &rocket::State<Config>,
) -> Json<Result<Output, Error>> {
    Json(run(data.0.code, semaphore, config).await)
}
