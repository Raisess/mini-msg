extern crate serde_json;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
  pub id: String,
  pub from: String,
  pub content: String,
}

impl Message {
  pub fn from_str(message: &str) -> Result<Message, serde_json::Error> {
    return serde_json::from_str(message);
  }

  pub fn to_string(message: &Message) -> String {
    return serde_json::to_string(message).expect("Invalid JSON");
  }
}

pub fn mount_message(from: &str, content: &str) -> Message {
  return Message {
    id: Uuid::new_v4().to_string(),
    from: from.to_string(),
    content: content.to_string(),
  };
}

pub trait Transporter {
  fn publish(&mut self, from_node_id: &str, to_node_id: &str, content: &str) -> ();

  fn consume(&mut self, node_id: &str) -> Option<Message>;
}
