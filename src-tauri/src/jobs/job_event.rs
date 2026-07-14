use serde::Serialize;

#[derive(Clone, Serialize)]
// #[serde(rename_all = "snake_case")]
pub enum UIAction {
    PasswordRequired
}

#[derive(Clone, Serialize)]
pub struct JobEvent {
    pub cmd_name: String,
    pub resource_path: String,
    pub created_at: String,
    pub ui_action: Option<UIAction>,
}
