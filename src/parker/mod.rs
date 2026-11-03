use crate::{Error, EngineDepotExt};

use salvo::prelude::*;




#[handler]
pub async fn init_default(req: &mut Request, depot: &mut Depot) -> Result<Text<String>, Error> {

    let domain = req.headers().get(salvo::http::header::HOST).and_then(|value| value.to_str().ok()).unwrap_or("");

    Ok(Text::Html(depot.index(&serde_json::json!({
        "domain": domain,
        "version": format!("v{}", env!("CARGO_PKG_VERSION")),
    }))?))
}




pub fn router() -> Router {
    Router::new()
        .push({
            Router::with_hoop(Compression::new().enable_gzip(CompressionLevel::Fastest))
                .push(Router::with_path("assets/{filename}").get(crate::routes::assets_endpoint))
                .push(Router::with_path("static/{filename}").get(crate::routes::assets_endpoint))
        })
        .push({
            Router::new()
                .push(Router::with_path("icons/{filename}").get(crate::routes::icons_endpoint))
        })
        .goal(init_default)
}