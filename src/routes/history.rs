use crate::{Error, EngineDepotExt, JsonDBDepotExt};


use salvo::prelude::*;




type HistoryList = Vec<History>;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct History {
    title: String,
    time: String,
    desc: String,
    space: Option<bool>
}


#[handler]
pub async fn init_default(depot: &mut Depot) -> Result<Text<String>, Error> {

    let history_list = depot.find_db::<HistoryList>("history.json").unwrap();

    let template = depot.template("history/base.hbs", serde_json::json!({
        "history": history_list,
    }))?;

    Ok(Text::Html(depot.index(&serde_json::json!({
        "content": template,
        "version": format!("v{}", env!("CARGO_PKG_VERSION")),
    }))?))

}