extern crate redis;

use crate::core::transporter::{Message, Transporter};

pub struct RedisTransporter {
  client: redis::Client,
}

impl RedisTransporter {
  pub fn new(host: &str, port: u16) -> Self {
    return Self {
      client: redis::Client::open(format!("redis://{}:{}", host, port))
        .expect("Redis connection to be open"),
    };
  }

  fn get_connection(&self) -> redis::Connection {
    return self.client.get_connection()
      .expect("Redis client to be connected");
  }
}

impl Transporter for RedisTransporter {
  fn publish(&mut self, from_node_id: &str, to_node_id: &str, content: &str) {
    let message = Message::new(from_node_id, content);

    let _: () = redis::cmd("LPUSH")
      .arg(&[to_node_id, &Message::to_string(&message)])
      .query(&mut self.get_connection())
      .expect("LPUSH to be executed");
  }

  fn consume(&mut self, node_id: &str) -> Option<Message> {
    if let Some(response) = redis::cmd("RPOP")
      .arg(node_id)
      .query::<Option<String>>(&mut self.get_connection())
      .expect("RPOP to be executed")
    {
      return match Message::from_str(&response) {
        Ok(message) => Some(message),
        Err(_) => None,
      };
    }

    return None;
  }
}
