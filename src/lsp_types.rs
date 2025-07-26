use std::collections::HashMap;

use base_types::LspAny;

pub mod base_types;
mod errors;

pub struct Message {
    // LSP is based in JSON-RPC and it always uses "2.0" as `jsonrpc` value
    jsonrpc: String,
    message_type: MessageType
}

impl Message {
    pub fn new() -> Message {
        return Message {
            // LSP is based in JSON-RPC and it always uses "2.0" as `jsonrpc` value
            jsonrpc: String::from("2.0"),
            message_type: MessageType::Test { test: true }
        };
    }

    pub fn as_json(&self) -> HashMap<String, LspAny> {
        let mut result_json: HashMap<String, LspAny> = HashMap::new();

        result_json.insert(
            String::from("jsonrpc"),
            LspAny::String(String::from(&self.jsonrpc))
        );

        return result_json;
    }

    pub fn as_string(&self) -> String {
        let message_json = self.as_json();
        let mut message_string = String::from("{");

        for (key, value) in message_json.iter() {
            let value_str = match value {
                LspAny::String(s) => format!("\"{s}\""),
                LspAny::Integer(i) => format!("{i}"),
                _ => String::new()
            };

            if message_string.len() != 1 {
                message_string.push_str(",");
            }

            message_string.push_str(
                format!("\n    \"{key}\": {value_str}").as_str()
            );
        }

        message_string.push_str("\n}");

        return message_string;
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
