use crate::lsp_types::Message;

pub fn encode_message(message: Message) -> String {
    // 1. Calculate size of message
    // 2. Construct the headers
    // 3. Add the content

    let message_as_json = message.as_json();
    // let message_as_str = String::from(message_as_json)

    let mut encoded_message = String::from("Content-Length: <NUMBER>");

    return encoded_message
}
