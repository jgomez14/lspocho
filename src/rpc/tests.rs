use super::*;

#[test]
fn encoding_test_message() {
    let test_message = Message::new();

    let encoded_message = encode_message(test_message);

    let expected_message = String::from(
        "Content-Length: 24\r\n\r\n{\n    \"jsonrpc\": \"2.0\"\n}"
    );

    assert_eq!(encoded_message, expected_message);
}
