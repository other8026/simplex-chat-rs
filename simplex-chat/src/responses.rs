pub use crate::types::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ChatResponse {
    ActiveUser {
        user: User,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatError {
        user_: Option<User>,
        chat_error: ChatError,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatCmdError {
        user_: Option<User>,
        chat_error: ChatError,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatRunning {
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatStarted {
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
    UserContactLink {
        user: User,
        contact_link: UserContactLink,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    UserContactLinkCreated {
        user: User,
        conn_req_contact: String,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    UsersList {
        users: Vec<UserInfo>,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    #[serde(untagged)]
    Unknown(JsonValue),
}
