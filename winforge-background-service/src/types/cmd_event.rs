#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CmdEvent {
    pub cmd_name: String,
    pub resource_path: String,
    pub resource_type: String,
    pub password: Option<String>,
}