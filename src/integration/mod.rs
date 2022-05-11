use serde::{Serialize, Deserialize};
use async_trait::async_trait;

mod proteus;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Archive {
    Proteus(proteus::ArchiveProteus),
    None,
}

impl Default for Archive {
    fn default() -> Self {
        Self::None
    }
}

impl Archive {
    pub async fn get_link(self, id: &str) -> Option<Result<String, String>> {
        match self {
            Self::Proteus(t) => Some(t.execute(id).await),
            Self::None => None,
        }
    }
}

#[async_trait]
pub(crate) trait ArchiveTraits {
    async fn execute(&self, id: &str) -> Result<String, String>;
}