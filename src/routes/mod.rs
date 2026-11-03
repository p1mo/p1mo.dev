mod catcher;
pub use catcher::init_catcher;

mod history;
mod home;
mod projects;

use crate::{Error, EngineDepotExt, JsonDBDepotExt, Language};

use salvo::prelude::*;

#[handler]
pub async fn assets_endpoint(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<(), Error> {
    let path = req.uri().path().strip_prefix("/").unwrap();
    if let Some(mime) = mime_guess::from_path(path).first() {
        res.add_header("Content-Type", mime.essence_str(), true)?;
    }
    Ok(res.write_body(depot.assets(path).unwrap())?)
}

#[handler]
pub async fn icons_endpoint(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<(), Error> {
    let path = req.uri().path().strip_prefix("/").unwrap();
    if let Some(mime) = mime_guess::from_path(path).first() {
        res.add_header("Content-Type", mime.essence_str(), true)?;
    }
    Ok(res.write_body(depot.icons(path).unwrap())?)
}

#[handler]
async fn skill_finder(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<(), Error> {
    let id = req.param::<String>("id").unwrap();

    let skill_list = depot.find_db::<Vec<Language>>("skills.json").unwrap();

    res.render(
        Text::Plain(
            match skill_list.iter().find(|item| item.uiid == id) {
                Some(skill) => skill.desc.clone(),
                None => "N/A".to_string(),
            }
        )
    );

    Ok(())
}

pub fn router() -> Router {
    Router::new()
        // ASSETS gzip Compressed
        .push({
            Router::with_hoop(Compression::new().enable_gzip(CompressionLevel::Minsize))
                .push(Router::with_path("assets/{filename}").get(assets_endpoint))
                .push(Router::with_path("static/{filename}").get(assets_endpoint))
        })
        // ASSETS no Compression
        .push({
            Router::new()
                .push(Router::with_path("icons/{filename}").get(icons_endpoint))
        })
        // Rest API
        .push({
            Router::new()
                .push(Router::with_path("skill/{id}").get(skill_finder))
        })
        // Website Routes
        .push({
            Router::with_hoop(Compression::new().enable_gzip(CompressionLevel::Minsize))
                .push(Router::new().get(home::init_default))
                .push(Router::with_path("history").get(history::init_default))
                .push(Router::with_path("projects").get(projects::init_default))
        })
}