use sha2::{Digest, Sha256};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CmdEvent {
    pub cmd_name: String,
    pub resource_path: String,
    pub resource_type: String,
    pub password: Option<String>
}

impl CmdEvent {
    pub fn get_id(&self) -> String {
        let input = format!(
            "{}{}{}",
            self.resource_path,
            self.resource_type,
            self.cmd_name
        );
        
        let mut sha256: Sha256 = Sha256::new();
        sha256.update(input);
        sha256.finalize()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }
}