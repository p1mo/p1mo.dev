use crate::{Error, EngineDepotExt};


use salvo::prelude::*;



#[handler]
async fn catch_all(&self, req: &Request, depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) -> Result<Text<String>, Error> {

    let path = req.uri().path();
    let method = req.method().as_str();

    let string = if let Some(status) = res.status_code {
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
    };

    let template = depot.template("errors.hbs", string)?;

    ctrl.skip_rest();

    Ok(Text::Html(depot.index(&serde_json::json!({
        "content": template,
        "version": format!("v{}", env!("CARGO_PKG_VERSION")),
    }))?))

}



pub fn init_catcher() -> Result<salvo::catcher::Catcher, Error> {
    Ok(salvo::catcher::Catcher::default()
        .hoop(affix_state::inject(crate::Engine::default()?))
        .hoop(catch_all))
}