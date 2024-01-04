use std::collections::HashMap;

use crate::core::transporter::{Message, Transporter, mount_message};

pub struct GenericTransporter {
  messages: HashMap<String, Vec<Message>>,
}

impl GenericTransporter {
  pub fn new() -> Self {
    return Self {
      messages: HashMap::new(),
    };
  }
}

impl Transporter for GenericTransporter {
  fn publish(
    &mut self,
    from_node_id: &str,
    to_node_id: &str,
    content: &str
  ) -> () {
    if let Some(vector) = self.messages.get_mut(to_node_id) {
      return vector.push(mount_message(from_node_id, content));
    }

    self.messages.insert(
      to_node_id.to_string(),
      Vec::from([mount_message(from_node_id, content)]),
    );
  }

  fn consume(&mut self, node_id: &str) -> Option<Message> {
    if let Some(vector) = self.messages.get_mut(node_id) {
      if vector.len() == 0 {
        self.messages.remove(node_id);
        return None;
      }

      return Some(vector.remove(0));
    }

    return None;
  }
}
