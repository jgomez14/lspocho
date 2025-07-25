use std::collections::HashMap;

use base_types::LspAny;

pub mod base_types;
mod errors;

pub struct Message {
    // LSP always uses "2.0" as `jsonrpc` value
    jsonrpc: String,
    message_type: MessageType
}

impl Message {
    pub fn as_json(&self) -> HashMap<String, LspAny> {
        let mut result_json: HashMap<String, LspAny> = HashMap::new();

        result_json.insert(
            String::from("jsonrpc"),
            LspAny::String(String::from(&self.jsonrpc))
        );

        return result_json
    }
}

enum MessageType {
    Request(RequestMessage),
    Response(ResponseMessage),
    Notification(NotificationMessage),
    Test { test: bool }
}

struct RequestMessage {
    id: base_types::LspRequestId,
    method: String,
    params: Option<base_types::LspObjectOrArray>
}

struct ResponseMessage {
    id: Option<base_types::LspRequestId>,
    response: Result<base_types::LspAny, ResponseError>
}

struct ResponseError {
    code: errors::ErrorCode,
    message: String,
    data: Option<base_types::LspAny>
}

struct NotificationMessage {
    method: String,
    params: Option<base_types::LspObjectOrArray>
}
