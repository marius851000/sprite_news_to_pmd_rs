use std::path::Path;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Default, Clone)]
pub struct MonsterId {
    pub path: Vec<String>,
}

impl MonsterId {
    pub fn from_path(path: &Path) -> Self {
        Self {
            path: path
                .iter()
                .map(|x| x.to_string_lossy().to_string())
                .collect(),
        }
    }

    pub fn to_slug(&self) -> String {
        self.path.join(&"-")
    }
}
