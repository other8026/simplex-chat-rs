pub use crate::types::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ChatResponse {
    ActiveUser {
        user: User,
    },
    UsersList {
        users: Vec<UserInfo>,
    },
    ChatStarted {
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatRunning {
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatStopped {
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Chats {
        // user: User,
        chats: Vec<Chat>,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    #[serde(untagged)]
    Unknown(JsonValue),
}
