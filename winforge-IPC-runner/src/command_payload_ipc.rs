use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandPayloadIPC {
    pub cmd_name: String,
    pub resource_path: String,
    pub password: Option<String>,
}