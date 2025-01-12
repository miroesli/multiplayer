use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::io::prelude::*;
use serde_json::json;

use crate::server_side::client::ClientID;

pub const MSG_SIZE: usize = 4096;
pub const TEXT_MESSAGE_IDENTIFIER: &str = "Text";
pub const REQUEST_CLIENT_ID_IDENTIFIER: &str = "RequestClientID";
pub const REQUEST_CLIENT_ID_RESPONSE_IDENTIFIER: &str = "RequestClientIDResponse";

/// Trait to define behaviour of a message. MSG_TYPE must be a unique identifier for the Message.
pub trait Message<'a>: Serialize + Deserialize<'a> {
    const MSG_TYPE: &'a str;

    /// Converts a Message to json with the following format: 
    /// 
    ///     {
    ///         "msg_type": MSG_TYPE,
    ///         "data": self,
    ///     }
    fn to_json_string(&self) -> String {

        let v = json!({
            "msg_type": Self::MSG_TYPE.to_owned(),
            "data": self,
        });

        v.to_string()
    }

    /// Converts a json string into a Message if possible.
    fn from_json_string(json_string: &'a str) -> serde_json::Result<Self> {

        let v_res:serde_json::Result<Self> = serde_json::from_str(json_string);
        match v_res {
            Ok(msg) => Ok(msg),
            Err(e) => Err(e),
        }

    }

}

#[derive(Deserialize, Serialize)]
/// Test messages are used for general communication
pub struct TextMessage {
    pub text: String,
}

#[derive(Deserialize, Serialize)]
/// Message to request a client identify themselves
pub struct RequestClientID;

#[derive(Deserialize, Serialize)]
/// Response to the RequestClientID message
pub struct RequestClientIDResponse {
    pub id: ClientID
}

impl Message<'static> for TextMessage{
    const MSG_TYPE: &'static str = TEXT_MESSAGE_IDENTIFIER;
}
impl Message<'static> for RequestClientID{
    const MSG_TYPE: &'static str = REQUEST_CLIENT_ID_IDENTIFIER;
}
impl Message<'static> for RequestClientIDResponse{
    const MSG_TYPE: &'static str = REQUEST_CLIENT_ID_RESPONSE_IDENTIFIER;
}

impl TextMessage{
    pub fn new<S: Into<String>>(text: S) -> TextMessage{
        TextMessage{
            text: text.into(),
        }
    }
}

/// Sends a generic message to a specified stream.
pub fn send_json<M: Message<'static>>(msg: M, socket: &mut TcpStream) {

    let json_string = msg.to_json_string();
    let buff = json_string.into_bytes();
    socket.write_all(&buff).expect("Failed to write to socket!");

}