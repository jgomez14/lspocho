use crate::lsp_types::Message;

#[cfg(test)]
mod tests;

pub fn encode_message(message: Message) -> String {
    let message_string = message.as_string();
    let message_bytes = message_string.len();

    let encoded_message = format!("Content-Length: {message_bytes}\r\n\r\n{message_string}");

    return encoded_message
}
