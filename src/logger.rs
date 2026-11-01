#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Kind {
    #[default]
    #[serde(rename = "info")]
    INFO,
    #[serde(rename = "debug")]
    DEBUG,
    #[serde(rename = "error")]
    ERROR,
    #[serde(rename = "trace")]
    TRACE,
}

impl Kind {
    pub fn from_str(input: &str) -> Self {
        match input {
            "info" => Self::INFO,
            "debug" => Self::DEBUG,
            "error" => Self::ERROR,
            "trace" => Self::TRACE,
            _ => Self::INFO,
        }
    }

    pub fn to_level(&self) -> tracing::Level {
        match self {
            Kind::INFO => tracing::Level::INFO,
            Kind::DEBUG => tracing::Level::DEBUG,
            Kind::ERROR => tracing::Level::ERROR,
            Kind::TRACE => tracing::Level::TRACE,
        }
    }
}

pub async fn tracing_logger(level_str: &str) {
    let level = Kind::from_str(level_str);
    tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_level(true)
        .with_max_level(level.to_level())
        .init()
}