use crate::{Error, EngineDepotExt, JsonDBDepotExt};


use salvo::prelude::*;





type ProjectList = Vec<ProjectListItems>;

type ProjectListItems = Vec<Project>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
struct Project {
    title: String,
    logo: Option<String>,
    site: String,
    desc: String,
    main: Option<bool>
}

#[handler]
pub async fn init_default(depot: &mut Depot) -> Result<Text<String>, Error> {

    let projects = depot.find_db::<ProjectList>("projects.json").unwrap();

    let rerender = {
        projects
            .iter()
            .map(|items| {
                items.iter().map(|item| {
                    if let Some(ref name) = item.logo {
                        match name.as_str() {
                            logo => {
                                return Some(Project {
                                    title: item.title.clone(),
                                    logo: match depot.assets(format!("__LOGO_{}__", logo.to_uppercase())) {
                                        Some(logo) => match String::from_utf8(logo.to_vec()).ok() {
                                            Some(svg) => Some(svg),
                                            None => None,
                                        },
                                        None => None,
                                    },
                                    site: item.site.clone(),
                                    desc: item.desc.clone(),
                                    main: item.main
                                });
                            }
                        }
                    }
                    Some(item.clone())
                }).collect::<Vec<Option<Project>>>()
            }).collect::<Vec<Vec<Option<Project>>>>()
    };

    let template = depot.template("projects/base.hbs", serde_json::json!({
        "projects": rerender
    }))?;

    Ok(Text::Html(depot.index(&serde_json::json!({
        "content": template,
        "version": format!("v{}", env!("CARGO_PKG_VERSION")),
    }))?))
}