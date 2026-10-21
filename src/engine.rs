use std::{collections::HashMap, sync::Arc};
use handlebars::Handlebars;





pub type ArcEngine<'a> = Arc<Engine<'a>>;

pub struct Engine<'a>(Handlebars<'a>, HashMap<String, &'static [u8]>);

impl<'a> Engine<'a> {

    pub fn default() -> Result<ArcEngine<'a>, crate::Error> {

        let mut engine = Handlebars::new();

        engine.register_template_string("__INDEX__", include_str!("../temp/app/index.html"))?;

        for entry in &ufs::bind_dir!("app/templates") {

            let path = entry.path.display().to_string();
            
            if !path.starts_with("assets") && path.ends_with(".hbs") || path.ends_with(".html") && path.ends_with(".html") {

                tracing::debug!("[TEMPATE] {}", path);

                engine.register_template_string(entry.path.to_str().unwrap(), String::from_utf8(entry.content()?.to_vec())?)?;

            }

        }

        let mut assets = HashMap::new();

        for entry in &ufs::bind_dir!("temp/app") {

            let path = entry.path.display().to_string();
            
            if path.starts_with("assets") {

                tracing::debug!("[ASSET] {}", path);

                assets.insert(entry.path.display().to_string(), entry.content()?);

            }

        }

        assets.insert("__LOGO_BLOCK__".to_string(), include_bytes!("../app/logos/blockz.svg"));
        assets.insert("__LOGO_KEOKU__".to_string(), include_bytes!("../app/logos/keoku.svg"));
        assets.insert("__LOGO_VEMOX__".to_string(), include_bytes!("../app/logos/vemox.svg"));
        assets.insert("__LOGO_VEXA__".to_string(), include_bytes!("../app/logos/vexa.svg"));

        Ok(ArcEngine::new(Self(engine, assets)))

    }

}





pub trait EngineDepotExt {

    fn index<T>(&self, data: T) -> Result<String, crate::Error>
    where
        T: serde::Serialize;
    
    fn assets<P>(&self, path: P) -> Option<&'static [u8]>
    where
        P: AsRef<std::path::Path>;
    
    fn template<P, T>(&self, path: P, data: T) -> Result<String, crate::Error>
    where
        P: AsRef<std::path::Path>,
        T: serde::Serialize;
    
}

impl EngineDepotExt for salvo::Depot {

    fn index<T>(&self, data: T) -> Result<String, crate::Error>
    where
        T: serde::Serialize
    {
        let engine = self.get_typed::<ArcEngine>().unwrap();
        let result = engine.0.render("__INDEX__", &data)?;
        Ok(result)
    }

    fn assets<P>(&self, path: P) -> Option<&'static [u8]>
    where
        P: AsRef<std::path::Path>
    {
        let engine = self.get_typed::<ArcEngine>().unwrap();
        Some(engine.1.get(path.as_ref().to_str().unwrap())?)
    }

    fn template<P, T>(&self, path: P, data: T) -> Result<String, crate::Error>
    where
        P: AsRef<std::path::Path>,
        T: serde::Serialize
    {
        let engine = self.get_typed::<ArcEngine>().unwrap();
        let result = engine.0.render(path.as_ref().to_str().unwrap(), &data)?;
        Ok(result)
    }

}
