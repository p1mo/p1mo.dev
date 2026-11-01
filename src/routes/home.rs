use crate::{Error, EngineDepotExt, JsonDBDepotExt, Language};


use salvo::prelude::*;




#[handler]
pub async fn init_default(depot: &mut Depot) -> Result<Text<String>, Error> {

    let skill_list = depot.find_db::<Vec<Language>>("skills.json").unwrap();

    let content = depot.template("home/base.hbs", serde_json::json!({
        "skill_list": skill_list,
        "programmer_svg"  : &String::from_utf8(depot.assets("__PROGRAMMER__").unwrap().to_vec())?,
        "langs_svg"  : &String::from_utf8(depot.assets("__LANGS__").unwrap().to_vec())?,
    }))?;

    Ok(Text::Html(depot.index(&serde_json::json!({
        "content": content,
        "version": format!("v{}", env!("CARGO_PKG_VERSION")),
    }))?))
}