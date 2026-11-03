use std::sync::Arc;

use crate::{EngineDepotExt, Error, Engine, ServerKind};

use salvo::prelude::*;



#[handler]
async fn catch_all(&self, req: &Request, depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) -> Result<Text<String>, Error> {

    let path = req.uri().path();
    let method = req.method().as_str();

    let engine = depot.get_typed::<Arc<ServerKind>>().unwrap();

    Ok(match engine.as_ref() {
        ServerKind::Parker => {

            ctrl.skip_rest();

            Text::Html(
                depot.index(&serde_json::json!({
                    "version": format!("v{}", env!("CARGO_PKG_VERSION")),
                }))?
            )

        },
        ServerKind::Website => {

            let template = depot.template("errors.hbs", if let Some(status) = res.status_code {
                serde_json::json!({
                    "code": status.as_str(),
                    "text": status.canonical_reason(),
                    "desc": format!("[{}] {}", method, path)
                })
            } else {
                serde_json::json!({
                    "code": "000",
                    "text": "unimplemented error",
                    "desc": format!("[{}] {}", method, path)
                })
            })?;

            ctrl.skip_rest();

            Text::Html(
                depot.index(&serde_json::json!({
                    "content": template,
                    "version": format!("v{}", env!("CARGO_PKG_VERSION")),
                }))?
            )

        },
    })


}









pub fn init_catcher(kind: ServerKind) -> Result<salvo::catcher::Catcher, Error> {
    Ok(
        salvo::catcher::Catcher::default()
            .hoop(affix_state::inject(Engine::default()?))
            .hoop(affix_state::inject(Arc::new(kind)))
            .hoop(catch_all)
    )
}