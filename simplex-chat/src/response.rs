use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalProfile {
    profile_id: u32,
    display_name: String,
    full_name: String,
    image: Option<String>,
    contact_link: Option<String>,
    local_alias: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    user_id: u32,
    agent_user_id: String,
    user_contact_id: u32,
    local_display_name: String,
    profile: LocalProfile,
    active_user: bool,
    // view_pwd_hash: String, // Declared in the typescript API, but not sent by server
    show_ntfs: bool,
    #[serde(flatten)]
    unknown_fields: HashMap<String, serde_json::Value>,
}
