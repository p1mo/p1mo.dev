mod engine;
pub use engine::{ArcEngine, EngineDepotExt};

mod github;

mod skills;
pub use skills::{parse_languages, Language, SkillsDepotExt};

use salvo::{prelude::*, writing::Text::Plain};



#[handler]
async fn assets(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<(), Error> {
    let path = req.uri().path().strip_prefix("/").unwrap();
    res.add_header("Content-Type", mime_guess::from_path(path).first().unwrap().essence_str(), true)?;
    Ok(res.write_body(depot.assets(path).unwrap())?)
}

#[handler]
async fn skill_finder(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<(), Error> {
    let id = req.param::<String>("id").unwrap();
    let skill = depot.find_skill(id).unwrap();
    res.render(Plain(skill.desc.clone()));
    Ok(())
}

#[handler]
async fn home(depot: &mut Depot) -> Result<Text<String>, Error> {

    let get_skills = depot.list_skills();

    Ok(Text::Html(depot.index(&serde_json::json!({
        "skills": get_skills,
        "logos": {
            "blockz": &String::from_utf8(depot.assets("__LOGO_BLOCK__").unwrap().to_vec())?,
            "keoku" : &String::from_utf8(depot.assets("__LOGO_KEOKU__").unwrap().to_vec())?,
            "vemox" : &String::from_utf8(depot.assets("__LOGO_VEMOX__").unwrap().to_vec())?,
            "vexa"  : &String::from_utf8(depot.assets("__LOGO_VEXA__").unwrap().to_vec())?,
        },
        "version": format!("v{}", env!("CARGO_PKG_VERSION")),
    }))?))
}



#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {

    tracing_subscriber::fmt()
    // Configure formatting settings.
    .with_target(false)
    .with_timer(tracing_subscriber::fmt::time::uptime())
    .with_level(true)
    .with_max_level(tracing::Level::INFO)
    // Set the subscriber as the default.
    .init();

    let arc_skills = parse_languages()?;
    let engine = engine::Engine::default()?;

    let acceptor = TcpListener::new("127.0.0.1:2222").bind().await;

    let router = Router::new()
        .hoop(affix_state::inject(engine))
        .hoop(affix_state::inject(arc_skills))
        .push({
            Router::new()
                .push(Router::with_path("assets/{filename}").get(assets))
                .push(Router::with_path("skill/{id}").get(skill_finder))
                .push(Router::with_path("/").get(home))
        });

    println!("{router:?}");

    let server = Server::new(acceptor);

    let handle = server.handle();

    tokio::spawn(listen_shutdown_signal(handle));

    server.serve(router).await;

    Ok(())
}

async fn listen_shutdown_signal(handle: salvo::server::ServerHandle) {
    // Wait Shutdown Signal
    let ctrl_c = async {
        // Handle Ctrl+C signal
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        // Handle SIGTERM on Unix systems
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(windows)]
    let terminate = async {
        // Handle Ctrl+C on Windows (alternative implementation)
        tokio::signal::windows::ctrl_c()
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    // Wait for either signal to be received
    tokio::select! {
        _ = ctrl_c => println!("ctrl_c signal received"),
        _ = terminate => println!("terminate signal received"),
    };

    // Graceful Shutdown Server
    handle.stop_graceful(None);
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
