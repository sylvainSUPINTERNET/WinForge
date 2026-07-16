#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Folder {
    pub id: i64,
    pub uid: String,
    pub resource_path: String,
    pub prompt: Option<String>,
    pub created_at: String
}