use uuid::Uuid;

#[derive(Debug)]
pub struct Message {
  pub id: String,
  pub from: String,
  pub content: String,
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
