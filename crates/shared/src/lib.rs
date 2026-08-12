use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<String>,
    pub title: String,
    pub completed: bool,
}
