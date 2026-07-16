#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CmdEvent {
    pub cmd_name: String,
    pub resource_path: String,
    pub resource_type: String,
    pub password: Option<String>
}

impl CmdEvent {
    pub fn get_id(&self) -> String {
        String::from(self.resource_path.as_str()) + self.resource_type.as_str() + self.cmd_name.as_str()
    }
}