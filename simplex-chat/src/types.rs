use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalProfile {
    profile_id: u64,
    display_name: String,
    full_name: String,
    image: Option<String>,
    contact_link: Option<String>,
    local_alias: String,
    #[serde(flatten)]
    _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    user_id: u64,
    agent_user_id: String,
    user_contact_id: u64,
    local_display_name: String,
    profile: LocalProfile,
    active_user: bool,
    // view_pwd_hash: String, // Declared in the typescript API, but not sent by server
    show_ntfs: bool,
    #[serde(flatten)]
    _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    user: User,
    unread_count: u64,
    #[serde(flatten)]
    _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    contact_id: u64,
    local_display_name: String,
    // profile: Profile,
    // active_conn: Connection,
    via_group: Option<u64>,
    // created_at: Date,
    #[serde(flatten)]
    _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ChatInfo {
    Direct {
        contact: Contact,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Group {
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ContactRequest {
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    #[serde(untagged)]
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatItem {
    // chat_dir: CIDirection,
    #[serde(flatten)]
    _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    chat_info: ChatInfo,
    // chat_items: Vec<ChatItem>,
    // chat_stats: ChatStats,
    #[serde(flatten)]
    _unknown_fields: HashMap<String, JsonValue>,
}
