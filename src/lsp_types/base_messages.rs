pub struct Message {
    // LSP always uses "2.0" as `jsonrpc` value
    jsonrpc: String,
    message_type: MessageType
}

enum MessageType {
    Request(RequestMessage),
    Response(ResponseMessage),
    Notification(NotificationMessage)
}

struct RequestMessage {
    id: String
}

struct ResponseMessage {}

struct NotificationMessage {}
