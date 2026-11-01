#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Language {
    pub uiid: String,
    pub icon: String,
    pub name: String,
    pub desc: String,
    pub tags: Vec<String>
}