use std::sync::Arc;

use ufs::UnifiedFS;







pub type ArcJsonDB = Arc<JsonDB>;


pub struct JsonDB {
    inner: UnifiedFS,
}


impl JsonDB {
    
    pub fn new() -> ArcJsonDB {
        Arc::new(Self { inner: ufs::bind_dir!("database") })
    }

    pub fn find<'a, S>(&self, path: impl AsRef<std::path::Path>) -> Option<S>
    where 
        S: serde::de::Deserialize<'a>,
    {
        for entry in self.inner.iter() {
            if entry.path.ends_with(path.as_ref()) {
                return Some(serde_json::from_slice(entry.content().ok()?).ok()?)
            }
        }
        None
    }

}




pub trait JsonDBDepotExt {
    
    fn find_db<'a, S>(&self, path: impl AsRef<std::path::Path>) -> Option<S>
    where
        S: serde::de::Deserialize<'a>;
    
}

impl JsonDBDepotExt for salvo::Depot {

    fn find_db<'a, S>(&self, path: impl AsRef<std::path::Path>) -> Option<S>
    where
        S: serde::de::Deserialize<'a>
    {
        let engine = self.get_typed::<ArcJsonDB>().unwrap();
        engine.find(path)
    }

}
