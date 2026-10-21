use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Language {
    pub uiid: String,
    pub name: String,
    pub desc: String,
    pub tags: Vec<String>
}

pub type Languages = Vec<Language>;

pub fn parse_languages() -> Result<Arc<Languages>, crate::Error> {
    let raw = include_bytes!("skills.json");
    let parsed: Languages = serde_json::from_slice(raw)?;
    Ok(Arc::new(parsed))
}




pub trait SkillsDepotExt {
    fn list_skills(&self) -> Vec<Language>;

    fn find_skill<P>(&self, path: P) -> Option<&Language>
    where
        P: AsRef<std::path::Path>;
}

impl SkillsDepotExt for salvo::Depot {
    fn list_skills(&self) -> Vec<Language> {
        let skills = self.get_typed::<Arc<Languages>>().unwrap();
        skills.iter().cloned().collect::<Vec<Language>>()
    }

    fn find_skill<P>(&self, path: P) -> Option<&Language>
    where
        P: AsRef<std::path::Path>
    {
        let skills = self.get_typed::<Arc<Languages>>().unwrap();
        skills.iter().find(|item| *item.uiid == *path.as_ref())
    }
}
