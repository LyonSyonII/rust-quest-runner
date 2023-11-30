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

    let config = match envy::from_env() {
        Ok(config) => config,
        Err(e) => panic!("{e}"),
    };
    println!("{config:#?}");
    let Config {
        authorization,
        port,
        semaphore_permits,
        semaphore_wait,
        kill_timeout,
        origins_whitelist,
        content_length_limit,
    } = config;

    if authorization.is_empty() {
        println!(
            "Warning: AUTH environment variable is not set, anyone will be able to send requests!"
        );
    }
    if origins_whitelist.is_empty() {
        println!(
            "Warning: ORIGINS_WHITELIST environment variable is not set, anyone will be able to send requests!"
        );
    }

    // Necessary for passing it into `auth`
//     let authorization: &'static str = authorization.leak();

    /*     let semaphore: &'static tokio::sync::Semaphore = Box::leak(Box::new(
        tokio::sync::Semaphore::new(semaphore_permits as usize),
    ));
    let cors = if origins_whitelist.is_empty() {
        warp::cors().allow_any_origin()
    } else {
        warp::cors().allow_origins(origins_whitelist.iter().map(String::as_str))
    }
    .allow_method(warp::http::Method::POST)
    .allow_headers(["content-type"]);

    let route = warp::post().and(warp::path("evaluate.json"));
    let auth = warp::header::header("authorization")
        .and_then(move |auth: String| async move {
            (auth == authorization)
                .then_some(())
                .ok_or(Error::not_authorized())
        })
        .untuple_one()
        .or_else(move |_| async move {
            authorization
                .is_empty()
                .then_some(())
                .ok_or(Error::not_authorized())
        });

    let process_input = warp::body::content_length_limit(content_length_limit as u64)
        .and(warp::body::json())
        .or_else(|_| async move { Err(Error::body_not_correct()) });
    let run_input = move |i: Input| run(i.code, semaphore, semaphore_wait, kill_timeout);

    let filter = route
        .and(auth)
        .and(process_input)
        .and_then(run_input)
        .recover(handle_rejection)
        .with(cors); */
    
//     println!("Listening on http://0.0.0.0:{port}/evaluate.json");

    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(rocket::figment::providers::Env::prefixed("APP_").global());
    rocket::custom(figment)
        .mount("/", rocket::routes![default_route, evaluate])
}

#[rocket::get("/evaluate.json")]
fn default_route() -> &'static str {
    "Waiting requests!"
}

#[rocket::post("/evaluate.json", format = "json", data = "<data>")]
async fn evaluate(data: Json<Input>) -> Json<Result<Output, Error>> {
    Json(run(data.0.code).await)
}
