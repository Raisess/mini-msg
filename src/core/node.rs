use uuid::Uuid;

use crate::core::transporter::{Message, Transporter};

pub struct Node {
  id: String,
}

impl Node {
  pub fn new(custom_id: Option<&str>) -> Self {
    let id = match custom_id {
      Some(id) => id.to_string(),
      None => Uuid::new_v4().to_string(),
    };

    return Self {
      id,
    };
  }

  pub fn id(&self) -> &str {
    return &self.id;
  }

  pub fn send(&self, transporter: &mut dyn Transporter, to: &str, content: &str) -> () {
    transporter.publish(self.id(), to, content);
  }

  pub fn read(&self, transporter: &mut dyn Transporter) -> Option<Message> {
    return transporter.consume(&self.id);
  }
}
