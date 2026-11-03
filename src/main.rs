mod config;
mod engine;
mod github;
mod routes;
mod server;
mod skills;
mod logger;

mod jsondb;
mod parker;

pub use engine::ArcEngine;
pub use engine::Engine;
pub use engine::EngineDepotExt;

pub use server::webserver;
pub use server::ServerKind;

pub use skills::Language;

pub use logger::Kind;

pub use jsondb::JsonDB;
pub use jsondb::JsonDBDepotExt;



use salvo::prelude::*;






#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {

    let env_level = std::env::var("P1MO_LOG_LEVEL").unwrap_or("info".to_string());

    logger::tracing_logger(&env_level).await;

    let conf = config::load_config(std::env::var("P1MO_CONFIG").unwrap_or("config.json".to_string()))?;


    let website = tokio::spawn({
        webserver(ServerKind::Website, format!("{}:{}", conf.host, conf.port), {
            Router::new()
                .hoop(affix_state::inject(Engine::default()?))
                .hoop(affix_state::inject(jsondb::JsonDB::new()))
                .push(routes::router())
        })
    });

    let parker_join = match conf.parker {
        Some(park) => {
            Some(tokio::spawn({
                webserver(ServerKind::Parker, format!("{}:{}", park.host, park.port), {
                    Router::new()
                        .hoop(affix_state::inject(Engine::default()?))
                        .hoop(affix_state::inject(jsondb::JsonDB::new()))
                        .push(parker::router())
                })
            }))
        },
        None => None,
    };

    let (wres, pres) = tokio::join!(website, server::optional_task(parker_join));

    tracing::info!("webserver: {:?}", wres??);
    tracing::info!("parking:   {:?}", pres?);

    Ok(())

}





#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    ParseUrlError(#[from] url::ParseError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    TemplateError(#[from] handlebars::TemplateError),
    #[error(transparent)]
    RenderError(#[from] handlebars::RenderError),
    #[error(transparent)]
    UfsError(#[from] ufs::Error),
    #[error(transparent)]
    SalvoError(#[from] salvo::Error),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),

    #[error("{0}")]
    Other(String),
}

#[async_trait]
impl Writer for Error {

    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render("custom error");
    }

}
